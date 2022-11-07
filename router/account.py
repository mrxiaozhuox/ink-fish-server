from fastapi import APIRouter

router = APIRouter(prefix="/account")

@router.get("/test")
async def account_test_api():
    return { "message": "Hello World" }