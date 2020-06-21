import sqlalchemy
from sqlalchemy.dialects import postgresql
import uuid

metadata = sqlalchemy.MetaData()

# Projects table
projects = sqlalchemy.Table(
    "projects",
    metadata,
    sqlalchemy.Column("id", postgresql.UUID, primary_key=True, default=uuid.uuid4(), unique=True, nullable=False),
    sqlalchemy.Column("user_id", postgresql.UUID, nullable=False),
    sqlalchemy.Column("name", sqlalchemy.String, nullable=False),
    sqlalchemy.Column("description", sqlalchemy.Text, nullable=False),
    sqlalchemy.Column("created_at", sqlalchemy.DateTime, nullable=False),
    sqlalchemy.Column("updated_at", sqlalchemy.DateTime)
)

# Deployments table
deployments = sqlalchemy.Table(
    "deployments",
    metadata,
    sqlalchemy.Column("id", postgresql.UUID, primary_key=True, default=uuid.uuid4, unique=True, nullable=False),
    sqlalchemy.Column("project_id", postgresql.UUID, nullable=False),
    sqlalchemy.Column("version", sqlalchemy.String, nullable=False),
    sqlalchemy.Column("hash", sqlalchemy.String, nullable=False),
    sqlalchemy.Column("has_static", sqlalchemy.Boolean, nullable=False),
    sqlalchemy.Column("published_at", sqlalchemy.DateTime, nullable=False)
)

# Routes table
routes = sqlalchemy.Table(
    "routes",
    metadata,
    sqlalchemy.Column("id", postgresql.UUID, primary_key=True, default=uuid.uuid4, unique=True, nullable=False),
    sqlalchemy.Column("deployment_id", postgresql.UUID, nullable=False),
    sqlalchemy.Column("path", sqlalchemy.Text, nullable=False),
    sqlalchemy.Column("methods", postgresql.ARRAY(sqlalchemy.Text), nullable=False),
    sqlalchemy.Column("handler", sqlalchemy.Text, nullable=False)
)

# Handlers table
handlers = sqlalchemy.Table(
    "handlers",
    metadata,
    sqlalchemy.Column("id", postgresql.UUID, primary_key=True, default=uuid.uuid4, unique=True, nullable=False),
    sqlalchemy.Column("deployment_id", postgresql.UUID, nullable=False),
    sqlalchemy.Column("name", sqlalchemy.Text, nullable=False),
    sqlalchemy.Column("query_parameters", postgresql.ARRAY(sqlalchemy.Text)),
    sqlalchemy.Column("headers", postgresql.ARRAY(sqlalchemy.Text)),
    sqlalchemy.Column("path_parameters", postgresql.ARRAY(sqlalchemy.Text)),
    sqlalchemy.Column("body", postgresql.JSONB(none_as_null=True)),
    sqlalchemy.Column("logic", postgresql.JSONB(none_as_null=True), nullable=False)
)
