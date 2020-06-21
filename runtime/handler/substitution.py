import re

TOKEN_START = "{{"
TOKEN_END = "}}"
REPLACE_REGEX = re.compile(rf"({TOKEN_START}[a-zA-Z0-9.]+{TOKEN_END})")


def split_tokens(raw_str):
    """
    Retrieve the individual tokens from a string

    :param raw_str: a string with potential replacements
    :return: array of tokens
    """
    return REPLACE_REGEX.split(raw_str)


def get_value_in_state(key, state):
    """
    Retrieve a potentially nested value in an object

    :param key: the `.` separated key (i.e: a.b.c.d)
    :param state: the object to access the key in
    :return: the value in the state
    """
    def traverse(hierarchy, current):
        level = hierarchy.pop(0)
        if level not in current:
            raise KeyError(f"specified key '{level}' does not exist")
        elif len(hierarchy) == 0:
            return current[level]
        elif type(current[level]) != dict:
            raise KeyError(f"cannot index non-dict item '{level}'")
        else:
            return traverse(hierarchy, current[level])

    keys = key.split(".")
    return traverse(keys, state)


def substitute(string, state):
    """
    Substitute a string with values from a state dictionary

    :param string: the string to replace values in
    :param state: the source replacement values
    :return: the replaced string
    """
    # Retrieve potential tokens
    tokenized = split_tokens(string)
    tokenized = list(filter(lambda v: v != '', tokenized))

    replaced = []
    for i, token in enumerate(tokenized):
        # Ensure value is token
        if not token.startswith("{{") and not token.endswith("}}"):
            replaced.append(token)
            continue

        # Get path to token
        key = token.replace("{{", "").replace("}}", "")

        # Get the token's value
        try:
            value = get_value_in_state(key, state)
        except KeyError:
            replaced.append(token)
            continue

        # Handle replacement if there's only one key and result is a dict/list
        if len(tokenized) == 1 and (type(value) is list or type(value) is dict):
            return value
        elif len(tokenized) == 1 and type(value) is bool:
            return value

        replaced.append(str(value))

    return ''.join(replaced)


def traverse_substitute(value, state):
    """
    Traverse some object and substitute variables when necessary

    :param value: the initial value
    :param state: the request state
    """
    # Check for replacement
    if type(value) is str:
        return substitute(value, state)

    # Traverse a dictionary for values
    elif type(value) is dict:
        for k, e in value.items():
            value[k] = traverse_substitute(e, state)
        return value

    # Traverse a list for values
    elif type(value) is list:
        for i, e in enumerate(value):
            value[i] = traverse_substitute(e, state)
        return value

    # Not replaceable/traversable, skip it
    else:
        return value
