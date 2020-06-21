FROM python:3.8-buster

# Copy required files
COPY . ./

# Install dependencies
RUN pip install -r requirements.txt

# Run the service
USER 1000
CMD ["gunicorn", "-w", "4", "-k", "uvicorn.workers.UvicornWorker", "-b", "0.0.0.0:9090", "main:app"]
