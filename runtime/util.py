import logging
import uuid


def convert_to(value: str, cls):
    """
    Attempt to convert a value to a specified type

    :param value: string to convert from
    :param cls: class to convert to
    :return: converted value or ``None``
    """
    try:
        return cls(value)
    except (ValueError, TypeError):
        return None


def set_config_var(from_file, from_env, default):
    """
    Set a configuration value based on the hierarchy of:
        default => file => env

    :param from_file: value from the configuration file
    :param from_env: value from the environment
    :param default: default configuration value
    :return: value to use as configuration
    """

    if from_env is not None:
        return from_env
    elif from_file is not None:
        return from_file
    else:
        return default


def generate_name(route, deployment):
    """
    Generate the name for a route in a given deployment

    :param route: the id of the route
    :param deployment: the id of the route
    :return: the unique name for the route
    """
    return f"{route}_{deployment}"


def find_route_by_name(route, deployment, app):
    """
    Find a route by its name

    :param route: the id of the route
    :param deployment: the id of the deployment the route is in
    :param app: starlette instance to retrieve route from
    :return: route and its corresponding endpoint
    """
    unique_name = generate_name(route, deployment)
    for i, route in enumerate(app.router.routes):
        if route.name == unique_name:
            return route, i
    return None, -1


def parse_uuid(s):
    """
    Attempt to parse a string as a UUID

    :param s: the potential uuid
    :return: the parsed uuid or `None`
    """
    try:
        return uuid.UUID(s)
    except ValueError:
        logging.error(f"Failed to parse uuid: {s}")
        return None
