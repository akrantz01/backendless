CREATE TABLE "deployments" (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    version VARCHAR(12) NOT NULL,
    hash CHAR(64) NOT NULL,
    has_static BOOLEAN NOT NULL,
    published_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE TABLE "routes" (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    deployment_id UUID NOT NULL REFERENCES deployments(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    methods TEXT[] NOT NULL,
    handler TEXT NOT NULL
);

CREATE TABLE "handlers" (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    deployment_id UUID NOT NULL REFERENCES deployments(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    query_parameters TEXT[],
    headers TEXT[],
    path_parameters TEXT[],
    body JSONB,
    logic JSONB NOT NULL
);
