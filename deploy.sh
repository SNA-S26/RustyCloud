#!/bin/bash
# Exit on error
set -e

echo "Starting deployment..."

# 1. Pull the latest image from Docker Hub
docker pull dambaya/rustycloud:latest

# 2. Stop and remove the old container if it exists
docker stop rustycloud || true
docker rm rustycloud || true

# 3. Run the new container:
docker run -d \
  --name rustycloud \
  --env-file .env \
  -v /mnt/nfs_share:/app/storage \
  -p 8080:80 \
  --restart always \
  dambaya/rustycloud:latest

# 4. Cleanup old images to save space
docker image prune -f

echo "Deployment finished successfully!"