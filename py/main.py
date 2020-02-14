import asyncio

# pausable
async def hello() -> str:
    return "world"

# pause
print(asyncio.get_event_loop().run_until_complete(hello()))