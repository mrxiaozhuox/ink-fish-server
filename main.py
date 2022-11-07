from fastapi import FastAPI
from router import account
from config import AppConfigure

from database.connection import init_database_connection

APP_CONFIGURE: AppConfigure = init_database_connection()

app = FastAPI()

app.include_router(account.router)

@app.get("/")
async def root():
    return { 'message': 'hello world' }