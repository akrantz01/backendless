from databases import Database
from dotenv import load_dotenv
from google.cloud import storage
from starlette.applications import Starlette
from starlette.responses import JSONResponse
import uvicorn

from config import Config
import pubsub

load_dotenv()


# Load configuration from sources
cfg = Config()

# Connect to database
database = Database(cfg.database_url)

# Connect to Google Cloud services
storage_client = storage.Client()
bucket = storage_client.bucket(cfg.gcp_bucket)


# Create API server
app = Starlette(
    routes=[],
    exception_handlers={
        404: lambda req, exc: JSONResponse({"success": False, "reason": "not found"}),
        405: lambda req, exc: JSONResponse({"success": False, "reason": "method not allowed"}),
        500: lambda req, exc: JSONResponse({"success": False, "reason": "internal server error"})
    },
    on_startup=[database.connect],
    on_shutdown=[database.disconnect]
)

# Add redis configuration to startup and shutdown
app.router.on_startup.append(pubsub.configure(app, cfg.redis))
app.router.on_shutdown.append(pubsub.shutdown(app))

# Attach services to state
app.state.database = database
app.state.bucket = bucket

if __name__ == "__main__":
    uvicorn.run("main:app", **cfg.app)
