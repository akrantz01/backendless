import os
import toml
import util


def read_from_environment():
    """
    Read configuration values from environment variables

    :return: configuration values
    """

    # Get all values
    server_host = os.environ.get("RUNTIME_SERVER_HOST")
    server_port = os.environ.get("RUNTIME_SERVER_PORT")
    server_workers = os.environ.get("RUNTIME_SERVER_WORKERS")
    redis_host = os.environ.get("RUNTIME_REDIS_HOST")
    redis_port = os.environ.get("RUNTIME_REDIS_PORT")
    redis_database = os.environ.get("RUNTIME_REDIS_DATABASE")
    database_url = os.environ.get("RUNTIME_DATABASE_URL")
    gcp_bucket = os.environ.get("RUNTIME_GCP_BUCKET")

    # Mark as non existent if blank
    if server_host == "":
        server_host = None
    if redis_host == "":
        redis_host = None
    if database_url == "":
        database_url = None
    if gcp_bucket == "":
        gcp_bucket = None

    # Try converting to integer
    server_port = util.convert_to(server_port, int)
    server_workers = util.convert_to(server_workers, int)
    redis_port = util.convert_to(redis_port, int)
    redis_database = util.convert_to(redis_database, int)

    return server_host, server_port, server_workers, redis_host, redis_port, redis_database, database_url, gcp_bucket


def determine_config_file():
    """
    Find a configuration file with the name ``settings.toml`` in the working directory.
    """

    # Filter files based on name
    files = [file for file in os.listdir(".") if os.path.isfile(file) and file == "settings.toml"]

    # Return the first file if one exists
    return files[0] if len(files) != 0 else None


def parse_from_file(raw_config: dict):
    """
    Parse the configuration and validate the types from the raw dictionary.

    :param raw_config: raw parsed configuration
    :return: parsed configuration
    """

    server_host, server_port, server_workers, \
        redis_host, redis_port, redis_database, \
        database_url, gcp_bucket = [None] * 8

    if type(raw_config.get("server")) is dict:
        raw_server = raw_config.get("server")  # type: dict

        if type(raw_server.get("host")) is str:
            server_host = raw_server.get("host")

        if type(raw_server.get("port")) is int:
            server_port = raw_server.get("port")

        if type(raw_server.get("workers")) is int:
            server_workers = raw_server.get("workers")

    if type(raw_config.get("redis")) is dict:
        raw_redis = raw_config.get("redis")  # type: dict

        if type(raw_redis.get("host")) is str:
            redis_host = raw_redis.get("host")

        if type(raw_redis.get("port")) is int:
            redis_port = raw_redis.get("port")

        if type(raw_redis.get("database")) is int:
            redis_database = raw_redis.get("database")

    if type(raw_config.get("database")) is dict:
        raw_database = raw_config.get("database")  # type: dict

        if type(raw_database.get("url")) is str:
            database_url = raw_database.get("url")

    if type(raw_config.get("gcp")) is dict:
        raw_gcs = raw_config.get("gcp")  # type: dict

        if type(raw_gcs.get("bucket")) is str:
            gcp_bucket = raw_gcs.get("bucket")

    return server_host, server_port, server_workers, redis_host, redis_port, redis_database, database_url, gcp_bucket


class Config(object):
    """
    Configure the server using environment variables and a TOML file
    """

    def __init__(self):
        # Check for configuration from a file
        file = determine_config_file()
        raw_file = {}
        if file is not None:
            raw_file = toml.load(open(file, "r"))

        # Parse the file configuration
        file_server_host, file_server_port, file_server_workers, \
            file_redis_host, file_redis_port, file_redis_database, \
            file_database_url, file_gcp_bucket = parse_from_file(raw_file)

        # Read from environment
        env_server_host, env_server_port, env_server_workers, \
            env_redis_host, env_redis_port, env_redis_database, \
            env_database_url, env_gcp_bucket = read_from_environment()

        # Set configuration
        self._host = util.set_config_var(file_server_host, env_server_host, "127.0.0.1")
        self._port = util.set_config_var(file_server_port, env_server_port, 9090)
        self._workers = util.set_config_var(file_server_workers, env_server_workers, 1)
        self._redis_host = util.set_config_var(file_redis_host, env_redis_host, "127.0.0.1")
        self._redis_port = util.set_config_var(file_redis_port, env_redis_port, 6379)
        self._redis_database = util.set_config_var(file_redis_database, env_redis_database, 0)
        self.database_url = util.set_config_var(file_database_url, env_database_url, "postgres://127.0.0.1:5432")
        self.gcp_bucket = util.set_config_var(file_gcp_bucket, env_gcp_bucket, "backendless-user-files")

    @property
    def app(self):
        return {"host": self._host, "port": self._port, "workers": self._workers}

    @property
    def redis(self):
        return {"address": f"redis://{self._redis_host}:{self._redis_port}", "db": self._redis_database}
