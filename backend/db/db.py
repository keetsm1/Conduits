import json
import sqlite3
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

from cryptography.fernet import Fernet
import keyring
from platformdirs import user_data_dir
APP_NAME = "Conduits"
KEYRING_USER = "sqlite_token_encryption_key"
db_dir = Path(user_data_dir(APP_NAME))
db_dir.mkdir(parents=True, exist_ok=True)
DB_PATH = db_dir / "connections.db"

def _get_or_create_cipher() -> Fernet:
    """Retrieves an encryption key from the OS native keychain or creates a new one."""
    key = keyring.get_password(APP_NAME, KEYRING_USER)
    if not key:
        key = Fernet.generate_key().decode()
        keyring.set_password(APP_NAME, KEYRING_USER, key)
    return Fernet(key.encode())


_cipher = _get_or_create_cipher()

def encrypt_str(plain_text: str) -> bytes:
    return _cipher.encrypt(plain_text.encode())


def decrypt_str(enc_bytes: bytes) -> str:
    return _cipher.decrypt(enc_bytes).decode()

def init_db():
    with sqlite3.connect(DB_PATH) as conn:
        conn.execute("""
            CREATE TABLE IF NOT EXISTS connections (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                provider TEXT NOT NULL DEFAULT 'google',
                account_email TEXT NOT NULL,
                google_sub TEXT UNIQUE,
                refresh_token_enc BLOB NOT NULL,
                scopes_json TEXT NOT NULL
            """)
        conn.commit()

def save_connectio(email: str, google_sub: str, refresh_token: str, scopes: List[str]):
    enc_refresh = encrypt_str(refresh_token)
    scopes_json = json.dumps(scopes)

    with sqlite3.connect(DB_PATH) as conn:
        conn.execute(""" 
        INSERT INTO connections (provider, account_email, google_sub, refresh_token_enc, scopes_json) 
        VALUES ('google',?,?,?,?)
        ON CONFLICT(google_sub) DO UPDATE SET
            account_email = excluded.account_email,
            refresh_token_enc = excluded.refresh_token_enc,
            scopes_json = excluded.scopes.json
            """, (email, google_sub, enc_refresh, scopes_json))
        conn.commit()