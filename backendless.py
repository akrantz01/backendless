import click
import fastjsonschema
import json
from pathlib import Path
import pickle
from requests import get, post, put, Session
import shutil
import sys
import yaml

USER = "akrantz01"
API_KEY = "35a8b4d294984df124763c51a0613e86637ccc5e5da3f7f76da193ca3e373a04"

session = Session()
try:
    with open("~/.backendless-session", "rb") as f:
        session.cookies.update(pickle.load(f))
except FileNotFoundError:
    pass


def download_schema(location):
    r = get(location)
    if r.status_code != 200:
        click.echo("Failed to retrieve JSON schema")
        sys.exit(1)
    return r.json()


def follow_references(raw, directory):
    # Basic validation for following references
    if type(raw) != dict:
        click.echo("Project file must be a dictionary")
        sys.exit(1)
    elif type(raw.get("routes")) != list:
        click.echo("'routes' key in project file must be a list")
        sys.exit(1)
    elif type(raw.get("handlers")) != list:
        click.echo("'handlers' key in project file must be a list")
        sys.exit(1)

    # Find references in routes definitions
    for i, item in enumerate(raw.get("routes")):
        if len(item) == 1 and "$ref" in item:
            with open(f"{directory}/{item['$ref']}", "r") as file:
                raw["routes"][i] = yaml.load(file, Loader=yaml.FullLoader)

    # Find references in handler definitions
    for i, item in enumerate(raw.get("handlers")):
        if len(item) == 1 and "$ref" in item:
            with open(f"{directory}/{item['$ref']}", "r") as file:
                raw["handlers"][i] = yaml.load(file, Loader=yaml.FullLoader)

    return raw


def get_full_project(project_file):
    return Path(project_file).resolve()


def validate_schema(location, project_file):
    # Ensure file exists
    if not project_file.exists():
        click.echo("Specified project file does not exist")
        sys.exit(1)

    schema = download_schema(location)

    # Read project file
    with open(project_file, "r") as file:
        parsed = yaml.load(file, Loader=yaml.FullLoader)

    # Convert references
    full = follow_references(parsed, str(project_file.parent))

    # Validate schema
    try:
        fastjsonschema.validate(schema, full)
    except fastjsonschema.JsonSchemaException as e:
        click.echo(f"Invalid project format: {e.message}")
        sys.exit(1)

    return full


@click.group()
@click.option("--schema", default="https://backendless.tech/schema.json", help="The project schema location")
@click.option("--server", default="https://api.backendless.tech", help="The API server to connect to")
@click.pass_context
def cli(ctx, server, schema):
    ctx.ensure_object(dict)
    ctx.obj["SERVER"] = server
    ctx.obj["SCHEMA"] = schema


@cli.command()
@click.argument("username")
@click.argument("email")
@click.option("--password", prompt=True, hide_input=True, confirmation_prompt=True)
@click.pass_context
def register(ctx, username, email, password):
    """Create your account"""
    r = post(f"{ctx.obj['SERVER']}/authentication/register", data={
        "username": username,
        "password": password,
        "email": email
    })
    if r.status_code != 200:
        click.echo(f"Failed to register: {r.json().reason}")
        sys.exit(1)
    click.echo(f"Successfully registered {email}")


@cli.command()
@click.argument("email")
@click.option("--password", prompt=True, hide_input=True)
@click.pass_context
def login(ctx, email, password):
    """Login to your account"""
    r = post(f"{ctx.obj['SERVER']}/authentication/login", data={
        "email": email,
        "password": password
    })
    if r.status_code != 200:
        click.echo(f"Failed to login: {r.json().reason}")
        sys.exit(1)
    click.echo(f"Successfully logged in as {email}")


@cli.command()
@click.argument("name")
@click.option("-d", "--description", default="none")
@click.pass_context
def new_project(ctx, name, description):
    """Create a new project"""
    r = post(f"{ctx.obj['SERVER']}/projects", data={
        "name": name,
        "description": description
    })
    if r.status_code != 200:
        click.echo(f"Failed to create new project: {r.json().reason}")
        sys.exit(1)
    click.echo(f"Successfully created new project '{name}'")


@cli.command()
@click.pass_context
def list_projects(ctx):
    """Get a list of your projects"""
    r = get(f"{ctx.obj['SERVER']}/projects")
    if r.status_code != 200:
        click.echo(f"Failed to list projects: {r.json().reason}")
        sys.exit(1)

    data = r.json()
    for project in data["data"]:
        click.echo(f"{project.id}: {project.name}")


@cli.command()
@click.argument("project_id")
@click.argument("project_file")
@click.pass_context
def deploy(ctx, project_id, project_file):
    """Deploy your project"""
    # Ensure schema is valid
    full_path = get_full_project(project_file)
    validated = validate_schema(ctx.obj["SCHEMA"], full_path)

    # Ensure specified static directory exists
    p = full_path.parent.joinpath(Path(validated.get("static_directory", "./static"))).resolve()
    if not p.exists():
        click.echo("Specified static directory does not exist")
        sys.exit(1)

    # Create zip archive of static files
    shutil.make_archive(full_path.parent.joinpath("static"), "zip", p)

    # Upload schema to server
    schema = post(f"{ctx.obj['SERVER']}/projects/{project_id}/deployments", data=validated)
    if schema.status_code != 200:
        click.echo(f"Failed to upload schema: {schema.json().reason}")
        sys.exit(1)

    # Upload static files to server
    upload = put(f"{ctx.obj['SERVER']}/projects/{project_id}/deployments",
                 files={"zip": open(full_path.parent.joinpath("static.zip"), "rb")})
    if upload.status_code != 200:
        click.echo(f"Failed to upload project: {upload.json().reason}")
        sys.exit(1)
    click.echo("Successfully uploaded project")


@cli.command()
@click.argument("project_file")
@click.pass_context
def validate(ctx, project_file):
    """Validate your project definition"""
    full_path = get_full_project(project_file)
    validate_schema(ctx.obj["SCHEMA"], full_path)
    click.echo("Successfully validated project configuration")


if __name__ == "__main__":
    cli()

    with open("~/.backendless-session", "wb") as f:
        pickle.dump(session.cookies, f)
