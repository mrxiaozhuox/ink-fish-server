import random
import time
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

DATABASE_URL = "sqlite:///./app.db"

engine = create_engine(DATABASE_URL, connect_args={"check_same_thread": False})
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

Base = declarative_base()

from . import account, configure

def get_database():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


def init_database_connection():
    account.Base.metadata.create_all(bind=engine)
    configure.Base.metadata.create_all(bind=engine)

    db = SessionLocal()

    # init configure default data
    if not configure.config_exists(db, "setup_time"):
        setup_time = int(time.time())
        configure.set_config(db, configure.ConfigureCreate(
            item="setup_time", value=str(setup_time)))

    hash_salt = configure.get_config(db, "hash_salt")
    if hash_salt is None:
        hash_salt = ''.join(random.sample(
            "abcdefghijklmnopqrstuvwxyz1234567890", 15))
        configure.set_config(db, configure.ConfigureCreate(
            item="hash_salt", value=hash_salt))

    db.close()
    # return AppConfigure(hash_slat=hash_salt)