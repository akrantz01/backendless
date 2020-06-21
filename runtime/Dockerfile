FROM python:3.8-alpine AS wheels
# Install dependencies for building
RUN apk add gcc musl-dev make postgresql-dev python3-dev build-base linux-headers

# Build requirements wheels
COPY requirements.txt ./
RUN pip wheel -r requirements.txt

FROM python:3.8-alpine

# Install built wheels
RUN apk add postgresql-dev
COPY --from=wheels /*.whl /
RUN pip install *.whl && rm -rf *.whl

# Define environment variables
ENV HOST 0.0.0.0
ENV PORT 9090

# Copy required files
COPY . ./

# Run the service
USER 1000
CMD gunicorn -w 4 -k uvicorn.workers.UvicornWorker -b ${HOST}:${PORT} main:app
