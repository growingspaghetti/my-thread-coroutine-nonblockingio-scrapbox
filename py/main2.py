import asyncio

loop = asyncio.get_event_loop()

# a bunch of future. pausable
future : asyncio.tasks._GatheringFuture = asyncio.gather(asyncio.sleep(1, "hello"), asyncio.sleep(2, " "), asyncio.sleep(3, "world"))

# <_GatheringFuture pending>
print(future)

# pause
print(''.join(loop.run_until_complete(future)))