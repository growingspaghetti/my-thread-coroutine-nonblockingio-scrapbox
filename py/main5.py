import asyncio

loop = asyncio.get_event_loop()

# a bunch of future. pausable
future : asyncio.tasks._GatheringFuture = asyncio.gather(asyncio.sleep(0, "world"))

# <_GatheringFuture pending> NOT `finished`
print(future)

# pause
print(''.join(loop.run_until_complete(future)))

# <_GatheringFuture finished result=['world']>
print(future)