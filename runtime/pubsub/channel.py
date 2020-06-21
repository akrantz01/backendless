import asyncio

from util import parse_uuid


async def reader(channel, app, worker):
    """
    Accept incoming messages from a pub/sub channel

    :param channel: the subscription channel to read from
    :param app: starlette app to modify routes on
    :param worker: function accepting a message and app instance
    """
    while await channel.wait_message():
        msg = (await channel.get()).decode()
        uuid = parse_uuid(msg)
        if uuid is None:
            continue

        asyncio.ensure_future(worker(app, uuid))
