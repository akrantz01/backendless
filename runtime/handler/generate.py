import fastjsonschema
from json.decoder import JSONDecodeError
from starlette.requests import Request
from starlette.responses import JSONResponse

from db import Handler
from .operations import OPERATIONS


def generate_handler(handler: Handler, project_id):
    """
    Generate a handler based on the specified configuration

    :param handler: handler configuration
    :param project_id: the id of the associated project
    :return: runnable handler
    """
    async def runner(request: Request):
        # Extract request parameters into the state
        state = {
            "query_parameters": dict(request.query_params),
            "path_parameters": dict(request.path_params),
            "headers": dict(request.headers),
            "body": await get_body(request),
            "cookies": request.cookies,
            "client": request.client.host
        }

        # Validate request body if required
        if handler.body is not None:
            if not validate_body(handler.body, state["body"]):
                return JSONResponse({"success": False, "reason": "invalid request body format"}, status_code=400)

        # Run through the specified logic
        for statement in handler.logic:
            action = statement.get("action")
            result = OPERATIONS[action](state, **statement, bucket=request.app.state.bucket, project_id=str(project_id))

            # Return result if exists
            if result is not None:
                return result

    return runner


async def get_body(r: Request):
    """
    Retrieve the JSON body of a request

    :param r: the raw request
    :return: the json body or an empty dictionary
    """
    try:
        return await r.json()
    except JSONDecodeError:
        return {}


def validate_body(schema: dict, body: dict) -> bool:
    """
    Validate the JSON body by the specified schema

    :param schema: the schema to validate against
    :param body: the body to be validated
    :return: whether the body is valid
    """
    try:
        fastjsonschema.validate(schema, body)
        return True
    except fastjsonschema.JsonSchemaException:
        return False
