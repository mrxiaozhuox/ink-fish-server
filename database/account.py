import hashlib
from sqlalchemy import Column, Integer, String, Boolean
from sqlalchemy.orm import Session
from .connection import Base

from pydantic import BaseModel

class UserModel(Base):
    
    __tablename__ = "users"

    id = Column(Integer, primary_key = True, index = True)

    email = Column(String, unique = True, index = True)
    username = Column(String)
    password = Column(String)
    
    bio = Column(String, default = 'Hello World!')
    location = Column(String, default = "")
    gender = Column(String, default = "unknown")

    is_active = Column(Boolean, default = True)

class UserBase(BaseModel):
    email: str

class UserCreate(UserBase):
    username: str
    password: str

class UserInfo(UserBase):
    id: int

    bio: str
    location: str
    gender: str
    
    class Config:
        orm_mode = True

def get_user(db: Session, id: int):
    return db.query(UserModel).filter(UserModel.id == id).first()

def get_user_by_email(db: Session, email: str):
    return db.query(UserModel).filter(UserModel.email == email).first()

def create_user(db: Session, user: UserCreate):
    db_user = UserModel(username=user.username, password=user.password, email=user.email)
    db.add(db_user)
    db.commit()
    db.refresh(db_user)
    return db_user