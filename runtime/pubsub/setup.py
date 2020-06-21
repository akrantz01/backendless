import aioredis
import asyncio

from .channel import reader
from .modifiers import delete_deployment, publish_deployment


def configure(app, config):
    """
    Configure redis pub/pub for an app

    :param app: app instance to be modified
    :param config: redis configuration to use
    """
    async def internal_configure():
        # Connect to redis and add to app state
        redis = await aioredis.create_redis(**config)
        app.state.redis = redis

        # Subscribe to the `publish` and `delete` channels
        delete, publish = await redis.subscribe("delete", "publish")

        # Run readers
        asyncio.ensure_future(reader(delete, app, delete_deployment))
        asyncio.ensure_future(reader(publish, app, publish_deployment))

    return internal_configure


def shutdown(app):
    """
    Gracefully shutdown redis

    :param app: app instance to retrieve redis connection from
    """
    async def internal_shutdown():
        # Retrieve redis
        redis = app.state.redis

        # Close the connection and wait
        redis.close()
        await redis.wait_closed()

    return internal_shutdown
