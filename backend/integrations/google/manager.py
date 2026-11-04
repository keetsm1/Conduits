import os

from dotenv import load_dotenv
from fastmcp import Client
from fastmcp.client.auth import OAuth

load_dotenv()

class GoogleMCPManager:
    def __init__(self):
        self.connections ={}

        self.endpoints = {
            "gmail": "https://gmailmcp.googleapis.com/mcp/v1"
        }

    async def connect(self, service, access_token=None):
        # access_token unused: OAuth flow below obtains its own token
        endpoint = self.endpoints[service]

        oauth = OAuth(
            client_id= os.getenv("GOOGLE_CLIENT_ID"),
            client_secret = os.getenv("GOOGLE_CLIENT_SECRET"),
            scopes= [
                "openid",
                "https://www.googleapis.com/auth/userinfo.email",
                "https://www.googleapis.com/auth/gmail.readonly",
                "https://www.googleapis.com/auth/gmail.compose",
            ],
            callback_port = 8765,
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

    async def call_tool(self, service, tool_name, arguments):
        return await self.connections[service].call_tool(
            tool_name,
            arguments
        )

    async def disconnect(self, service):
        client = self.connections.pop(service, None)

        if client:
            await client.__aexit__(None, None, None)