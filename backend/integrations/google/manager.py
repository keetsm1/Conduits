import os

from dotenv import load_dotenv
from fastmcp import Client
from fastmcp.client.auth import OAuth
from key_value.aio.stores.keyring import KeyringStore
from mcp.client.auth import oauth2 as _mcp_oauth2
from mcp.client.auth.utils import issuers_match as _issuers_match

load_dotenv()

# Google lists "https://accounts.google.com/" (trailing slash) in its protected
# resource metadata but "https://accounts.google.com" as the issuer. mcp 2.3.0
# only tolerates that slash on its legacy discovery path, so OAuth dies with
# "issuer mismatch" before the browser opens.
# shortcut: drop this patch once mcp normalizes the issuer on the PRM path.
_validate_issuer = _mcp_oauth2.validate_metadata_issuer


def _tolerant_issuer_check(oauth_metadata, expected_issuer):
    if not _issuers_match(str(oauth_metadata.issuer), expected_issuer):
        _validate_issuer(oauth_metadata, expected_issuer)


_mcp_oauth2.validate_metadata_issuer = _tolerant_issuer_check

class GoogleMCPManager:
    def __init__(self):
        self.connections ={}

        # Tokens live in the OS keychain, so a restart refreshes instead of
        # re-running the browser flow (fastmcp's default store is memory-only).
        self.token_storage = KeyringStore(service_name="Conduits-MCP")

        self.endpoints = {
            "gmail": "https://gmailmcp.googleapis.com/mcp/v1",
            "drive": "https://drivemcp.googleapis.com/mcp/v1"
        }

        self.scopes = {
            "gmail": [
                "openid",
                "https://www.googleapis.com/auth/userinfo.email",
                "https://www.googleapis.com/auth/gmail.readonly",
                "https://www.googleapis.com/auth/gmail.compose",
            ],
            "drive":[
                "openid",
                "https://www.googleapis.com/auth/userinfo.email",
                "https://www.googleapis.com/auth/drive.readonly",
                "https://www.googleapis.com/auth/drive.file"
            ]

        }

        # The MCP handshake and tools/list succeed unauthenticated, so a cheap
        # authenticated tool call is the only real proof the user consented.
        self.verify_tools = {
            "gmail": ("list_drafts", {"pageSize": 1}),
            "drive": ("list_recent_files", {"pageSize": 1}),
        }

    async def connect(self, service, access_token=None):
        # access_token unused: OAuth flow below obtains its own token
        endpoint = self.endpoints[service]

        oauth = OAuth(
            client_id= os.getenv("GOOGLE_CLIENT_ID"),
            client_secret = os.getenv("GOOGLE_CLIENT_SECRET"),
            scopes= self.scopes[service],
            token_storage=self.token_storage
        )
        

        client = Client(
            endpoint,
            auth = oauth
        )

        await client.__aenter__()

        self.connections[service]= client

        return client

    async def get_tools(self, service):
        return await self.connections[service].list_tools()

    async def verify(self, service):
        """Fail unless the user actually granted access."""
        tool, arguments = self.verify_tools[service]
        await self.call_tool(service, tool, arguments)

    async def call_tool(self, service, tool_name, arguments):
        return await self.connections[service].call_tool(
            tool_name,
            arguments
        )

    async def disconnect(self, service):
        client = self.connections.pop(service, None)

        if client:
            await client.__aexit__(None, None, None)