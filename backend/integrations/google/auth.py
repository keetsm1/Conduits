import json
import os
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
import httpx
from dotenv import load_dotenv

# Resolve backend/ regardless of the directory the script is run from
BACKEND_DIR = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(BACKEND_DIR))
load_dotenv(BACKEND_DIR / ".env")

import db


SCOPES = [
    "openid",
    "https://www.googleapis.com/auth/userinfo.email",
    "https://www.googleapis.com/auth/gmail.readonly",
    "https://www.googleapis.com/auth/gmail.compose",
]

def connect_account() -> str:
    #this runs the loopback OAuth Flow in the user's browser and save credentials to SQLite
    client_config = {
        "installed":{
            "client_id" : os.environ["GOOGLE_CLIENT_ID"],
            "client_secret" : os.environ["GOOGLE_CLIENT_SECRET"],
            "auth_uri":"https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token",
            "redirect_uris": ["http://localhost"],
        }
    }

    flow = InstalledAppFlow.from_client_config(client_config, SCOPES)
    creds = flow.run_local_server(port=0, prompt="consent", access_type="offline")

    # Fetch identity info using the temporary access token
    with httpx.Client() as client:
        res = client.get(
            "https://www.googleapis.com/oauth2/v3/userinfo",
            headers={"Authorization": f"Bearer {creds.token}"},
        )
        res.raise_for_status()
        user_info = res.json()

    email = user_info.get("email")
    google_sub = user_info.get("sub")
    if not email or not google_sub or not creds.refresh_token:
        raise RuntimeError("Google did not return an email, subject or refresh token")
    granted_scopes = list(creds.scopes) if creds.scopes else SCOPES

    db.save_connection(
        email=email,
        google_sub=google_sub,
        refresh_token=creds.refresh_token,
        scopes=granted_scopes,
    )
    return email

