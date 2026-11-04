import asyncio
from integrations.google.manager import GoogleMCPManager


async def main():

    manager = GoogleMCPManager()

    try:
        await manager.connect("gmail")

        tools = await manager.get_tools("gmail")

        for tool in tools:
            print(tool.name)

    finally:
        await manager.disconnect("gmail")


asyncio.run(main())