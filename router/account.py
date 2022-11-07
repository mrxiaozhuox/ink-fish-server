from fastapi import APIRouter, Depends, HTTPException

from database import account

from sqlalchemy.orm import Session
from database.connection import get_database

router = APIRouter(prefix="/account")

@router.get("/info/", response_model=account.UserInfo)
async def get_account_info(
    id: str | None = None,
    email: str | None = None, 
    db: Session = Depends(get_database)
    ):

    if id is not None:
        user = account.get_user(db, id)
    elif email is not None:
        user = account.get_user_by_email(db, email)
    else:
        user = None

    if user is None:
        raise HTTPException(status_code=400, detail="User not found")
    return user

@router.post("/register/", response_model=account.UserInfo)
def register_account(user: account.UserCreate, db: Session = Depends(get_database)):
    db_user = account.get_user_by_email(db, email=user.email)
    if db_user:
        raise HTTPException(status_code=400, detail="Email already registered")
    return account.create_user(db=db, user=user)