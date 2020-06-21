from starlette.routing import Route

from db import Deployment
from handler import generate_handler
from util import find_route_by_name, generate_name


async def publish_deployment(app, uuid):
    """
    Publish a deployment's routes

    :param app: app instance to be modified
    :param uuid: the id of the deployment
    """
    # Fetch deployment and its associated handlers and routes
    db = app.state.database
    deployment = await Deployment.find(uuid, db)
    handlers = await deployment.handlers
    routes = await deployment.routes
    project = await deployment.project

    # Generate handlers
    handlers_by_name = {}
    for handler in handlers:
        handlers_by_name[handler.name] = generate_handler(handler, deployment.project_id)

    # Add routes
    for route in routes:
        # Retrieve handler for route
        h = handlers_by_name.get(route.handler)
        if h is None:
            continue

        # Generate route
        name = generate_name(route.id, deployment.id)
        r = Route(
            f"/{project.name}/{deployment.version}{route.path}",
            endpoint=h,
            methods=route.methods,
            name=name
        )

        # Add route to app
        app.router.routes.append(r)


async def delete_deployment(app, uuid):
    """
    Delete a deployment's routes

    :param app: app instance to be modified
    :param uuid: the id of the deployment
    """
    # Fetch deployment and its associated routes
    db = app.state.database
    deployment = await Deployment.find(uuid, db)
    routes = await deployment.routes

    # Remove routes from app
    for route in routes:
        # Retrieve the route
        _, i = find_route_by_name(route.id, deployment.id, app)
        if i == -1:
            continue

        del app.router.routes[i]
