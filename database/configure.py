from sqlalchemy import Column, Integer, String, Text
from sqlalchemy.orm import Session
from .connection import Base

from pydantic import BaseModel

class ConfigureModel(Base):
    __tablename__ = "configures"

    id = Column(Integer, primary_key = True, index = True)
    item = Column(String, unique = True)
    value = Column(Text)

class ConfigureCreate(BaseModel):
    item: str
    value: str

class CongfigureInfo(ConfigureCreate):
    id: int

def get_config(db: Session, item: str):
    return db.query(ConfigureModel).filter(ConfigureModel.item == item).first()

def get_config_by_id(db: Session, id: int):
    return db.query(ConfigureModel).filter(ConfigureModel.id == id).first()

def set_config(db: Session, configure: ConfigureCreate):
    db_configure = ConfigureModel(item = configure.item, value = configure.value)
    db.add(db_configure)
    db.commit()
    db.refresh(db_configure)
    return db_configure