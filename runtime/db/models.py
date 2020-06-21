from .tables import deployments, handlers, routes


class Deployment(object):
    """
    Representation of a deployment in the database

    :param record: a record found in the database
    :param db: a database connection object
    """
    def __init__(self, record, db):
        self.__db = db
        self.id = record.get("id")
        self.project_id = record.get("project_id")
        self.version = record.get("version")
        self.hash = record.get("hash")
        self.has_static = record.get("has_static")
        self.published_at = record.get("published_at")

    def __str__(self):
        return f"<Deployment id={self.id} project_id={self.project_id} version={self.version}>"

    def __repr__(self):
        return self.__str__()

    @property
    async def routes(self):
        """
        Retrieve all routes associated with the deployment

        :return: array of routes
        """
        query = routes.select().where(routes.c.deployment_id == self.id)
        records = await self.__db.fetch_all(query=query)
        return [Route(record) for record in records]

    @property
    async def handlers(self):
        """
        Retrieve all handlers associated with the deployment

        :return: array of handlers
        """
        query = handlers.select().where(handlers.c.deployment_id == self.id)
        records = await self.__db.fetch_all(query=query)
        return [Handler(record) for record in records]

    @classmethod
    async def find(cls, deployment_id, db):
        """
        Find a deployment by its id

        :param deployment_id: the uuid of the deployment
        :param db: a database connection object
        """
        query = deployments.select().where(deployments.c.id == deployment_id)
        record = await db.fetch_one(query=query)
        return cls(record, db)

    @classmethod
    async def all(cls, db):
        """
        Find all deployments in the database

        :param db: a database connection object
        """
        query = deployments.select()
        records = await db.fetch_all(query=query)
        return [cls(record, db) for record in records]


class Route(object):
    """
    Representation of a route in the database

    :param record: a record found in the database
    """
    def __init__(self, record):
        self.id = record.get("id")
        self.deployment_id = record.get("deployment_id")
        self.path = record.get("path")
        self.methods = record.get("methods")
        self.handler = record.get("handler")

    def __str__(self):
        return f"<Route id={self.id} deployment_id={self.deployment_id} path={self.path}>"

    def __repr__(self):
        return self.__str__()


class Handler(object):
    """
    Representation of a handler in the database

    :param record: a record found in the database
    """
    def __init__(self, record):
        self.id = record.get("id")
        self.deployment_id = record.get("deployment_id")
        self.name = record.get("name")
        self.query_parameters = record.get("query_parameters")
        self.headers = record.get("headers")
        self.path_parameters = record.get("path_parameters")
        self.body = record.get("body")
        self.logic = record.get("logic")

    def __str__(self):
        return f"<Handler id={self.id} deployment_id={self.deployment_id} name={self.name}>"

    def __repr__(self):
        return self.__str__()
