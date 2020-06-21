from starlette.routing import Route

from db import Deployment
from handler import generate_handler
from util import generate_name


# This is the same as /runtime/pubsub/modifiers.py#publish_deployment
def load_routes(app, db):
    async def inner():
        deployments = await Deployment.all(db)

        for deployment in deployments:
            handlers = await deployment.handlers
            routes = await deployment.routes

            handlers_by_name = {}
            for handler in handlers:
                handlers_by_name[handler.name] = generate_handler(handler, deployment.project_id)

            for route in routes:
                h = handlers_by_name.get(route.handler)
                if h is None:
                    continue

                name = generate_name(route.id, deployment.id)
                r = Route(
                    f"/{deployment.version}{route.path}",
                    endpoint=h,
                    methods=route.methods,
                    name=name
                )

                app.router.routes.append(r)
    return inner
