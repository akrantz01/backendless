from base64 import b64encode
from google.cloud import datastore
import json
from os import urandom
from random import random
from starlette.responses import FileResponse, JSONResponse, PlainTextResponse
from tempfile import NamedTemporaryFile
from uuid import uuid4

from .substitution import substitute, traverse_substitute


def return_(state, *, data_type, value, status=200, **_):
    """
    Return a response with the given status, value, and data

    :param state: the request state
    :param data_type: the data type to return
    :param value: the value to return
    :param status: the status code to return
    """
    # Substitute values in return type
    if data_type == "json":
        if type(value) is not dict and type(value) is not list:
            value = substitute(value, state)
        else:
            value = traverse_substitute(value, status)
    else:
        value = substitute(value, state)

    return {
        "json": JSONResponse,
        "text": PlainTextResponse,
    }[data_type](value, status)


def math(state, *, operation, a, b, store=None, **_):
    """
    Run a given calculation

    :param state: the request state
    :param operation: the operation to run
    :param a: first parameter of the operation
    :param b: second parameter of the operation
    :param store: where to store the result
    """
    # Replace values
    a = substitute(a, state)
    b = substitute(b, state)

    # Ensure the values are numbers
    try:
        a = float(a)
    except ValueError:
        return JSONResponse({"success": False, "reason": f"parameter 'a' could not be converted to a number, got {a}"},
                            status_code=400)
    try:
        b = float(b)
    except ValueError:
        return JSONResponse({"success": False, "reason": f"parameter 'b' could not be converted to a number, got {b}"},
                            status_code=400)

    # Run the calculation
    result = {
        "add": lambda x, y: x + y,
        "subtract": lambda x, y: x - y,
        "multiply": lambda x, y: x * y,
        "divide": lambda x, y: x / y,
    }[operation](a, b)

    # Store it if needed
    if store is not None:
        state[store] = result


def db(state, *, operation, collection, project_id, data={}, query=None, store=None, **_):
    """
    Run a database operation with the given data

    :param state: the request state
    :param operation: database operation to run
    :param collection: collection to operate on
    :param project_id: the overarching project id
    :param data: the data to use
    :param query: the query to run
    :param store: where to store the result
    """
    # Connect to the database
    client = datastore.Client(namespace=project_id)

    # Replace values in query and data
    data = traverse_substitute(data, state)
    query = traverse_substitute(query, state)

    def create():
        # Create key and corresponding entity
        key = client.key(collection)
        entity = datastore.Entity(key)

        # Update the entity with the specified data
        entity.update(data)

        # Insert into the database
        client.put(entity)
        return dict(entity)

    def find_one():
        # Validate inputted query
        if type(query) is not str and type(query) is not int:
            return JSONResponse({"success": False,
                                 "reason": "query for 'find-one' must be either a replaceable string or integer"},
                                status_code=500)

        # Attempt to convert to integer
        key = substitute(query, state)
        try:
            key = int(key)
        except ValueError:
            pass

        # Run query for entity
        key = client.key(collection, key)
        entity = client.get(key)

        # Return existence or non-existence
        if entity is None:
            return False
        return {**dict(entity), "id": entity.id}

    def find_many():
        # Validate inputted query
        if type(query) is not list:
            return JSONResponse({"success": False,
                                 "reason": "query for 'find-many' must be an array of three-element arrays"},
                                status_code=500)

        # Construct query
        q = client.query(kind=collection)
        for filter_part in query:
            q.add_filter(filter_part[0], filter_part[1], substitute(filter_part[2], state))

        # Retrieve results
        return [{**dict(entity), "id": entity.id} for entity in q.fetch()]

    def update():
        # Validate inputted query
        if type(query) is not str and type(query) is not int:
            return JSONResponse({"success": False,
                                 "reason": "query for 'update' must be either a replaceable string or integer"},
                                status_code=500)

        # Attempt to convert to integer
        key = substitute(query, state)
        try:
            key = int(key)
        except ValueError:
            pass

        # Retrieve the entity
        key = client.key(collection, key)
        entity = client.get(key)

        # Ensure the entity exists
        if entity is None:
            return False

        # Update entity data
        entity.update(data)
        client.put(entity)

    def delete():
        # Validate inputted query
        if type(query) is not str and type(query) is not int:
            return JSONResponse({"success": False,
                                 "reason": "query for 'delete' must be either a replaceable string or integer"},
                                status_code=500)

        # Attempt to convert to integer
        key = substitute(query, state)
        try:
            key = int(key)
        except ValueError:
            pass

        # Delete the entity
        key = client.key(collection, key)
        client.delete(key)

    # Run operation
    store_value = {
        "create": create,
        "find-one": find_one,
        "find-many": find_many,
        "update": update,
        "delete": delete,
    }[operation]()

    # Return error if generated
    if type(store_value) is JSONResponse:
        return store_value

    # Store the database result if required
    if store_value is not None and store is not None:
        state[store] = store_value


def static(state, *, file, bucket, project_id, **_):
    """
    Serve a static file to the requester

    :param state: the request status
    :param file: the file to retrieve
    :param bucket: the GCP bucket reference
    :param project_id: the id of the project
    """
    file = substitute(file, state)
    if file.startswith("/"):
        blob = bucket.blob(f"{project_id}{file}")
    else:
        blob = bucket.blob(f"{project_id}/{file}")

    # Ensure exists
    if not blob.exists():
        return JSONResponse({"success": False, "reason": f"specified file ({file}) was not found"}, status_code=404)

    # Create and write to temporary file
    with NamedTemporaryFile() as file:
        blob.download_to_file(file)

        # Return the file
        return FileResponse(file.name)


def generator(state, *, data_type, store, **_):
    """
    Generate a value as a uuid, number, or string

    :param state: the request state
    :param data_type: what type of data to generate
    :param store: where to store the result
    """
    state[store] = {
        "uuid": lambda: uuid4(),
        "number": random(),
        "string": b64encode(urandom(8)).decode()
    }[data_type]()


def if_(state, *, conditional, true, false, **_):
    """
    Run a conditional statement

    :param state: the request state
    :param conditional: the conditional to evaluate
    :param true: the success array of statements
    :param false: the failure array of statements
    """
    conditional = substitute(conditional, state)

    # Evaluate conditional
    try:
        conditional_result = eval(conditional, {}, {})
    except Exception as e:
        return JSONResponse({"success": False, "reason": f"failed to evaluate conditional: {e}"})

    # Run statements for branch
    for statement in true if conditional_result else false:
        action = statement.get("action")
        result = OPERATIONS[action](state, **statement)

        # Return result if exists
        if result is not None:
            return result


def coerce(state, *, data_type, store=None, **_):
    """
    Forcibly convert from one type to another

    :param state: the request state
    :param data_type: the type to convert to
    :param store: where to store the result
    """
    try:
        state[store] = {
            "boolean": bool,
            "integer": int,
            "float": float
        }[data_type](state[store])
    except ValueError:
        return JSONResponse({"success": False, "reason": f"could not convert variable '{store}' to '{data_type}'"},
                            status_code=400)


OPERATIONS = {
    "return": return_,
    "math": math,
    "db": db,
    "static": static,
    "generator": generator,
    "if": if_,
    "coerce": coerce
}
