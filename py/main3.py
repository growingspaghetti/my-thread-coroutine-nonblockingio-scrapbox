# https://asyncio.readthedocs.io/en/latest/hello_world.html
import asyncio

async def non_blocking_sleep() -> None:
    await asyncio.sleep(4)

async def say(what, when) -> None:
    await asyncio.sleep(when)
    print(what)

loop = asyncio.get_event_loop()

task1 : asyncio.tasks.Task = loop.create_task(say('first hello', 2))
task2 : asyncio.tasks.Task = loop.create_task(say('second hello', 1))

# <Task pending coro=<say() running at main3.py:4>>
print(task1)
loop.run_until_complete(non_blocking_sleep())

future : asyncio.tasks._GatheringFuture = asyncio.gather(task1, task2)
loop.run_until_complete(future)