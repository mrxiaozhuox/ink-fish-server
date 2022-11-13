from fastapi import FastAPI, Depends
from router import account
from config import AppConfigure

from database.connection import init_database_connection

APP_CONFIGURE: AppConfigure = init_database_connection()

def get_config():
    return APP_CONFIGURE

app = FastAPI(dependencies=[Depends(get_config)])

@app.on_event("startup")
def app_startup():
    pass

app.include_router(account.router)

@app.get("/")
async def root():
    return { 'message': 'hello world' }