-- Your SQL goes here
CREATE TABLE users (
    id BIGINT PRIMARY KEY AUTOINCREMENT NOT NULL, 
    chat_id TEXT UNIQUE NOT NULL, 
    passwd TEXT NOT NULL, 
    email TEXT NOT NULL, 
    phone TEXT, 
    name TEXT NOT NULL, 
    profile_image BLOB, 
    sex INTEGER NOT NULL DEFAULT 0, 
    region TEXT NOT NULL, 
    personalized_signature TEXT
)