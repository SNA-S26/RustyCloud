#!/usr/bin/env bash
set -euo pipefail

# Secrets

set -a
source .env
set +a

# Variables

export MONGODB_URI="mongodb://${MONGODB_USER}:${MONGODB_PASS}@mongodb-service.database.svc.cluster.local:27017"
export REDIS_URI="redis://redis-service.database.svc.cluster.local:6379"
export RUSTYCLOUD_HOST="rustycloud.ru"
export IMAGE="docker.io/semyonnadutkin/rustycloud:latest"

# Check for k8s / k3s

echo "[1/8] Checking kubectl..."
kubectl version --client >/dev/null

echo "[2/8] Checking cluster..."
kubectl get nodes

# Check the namespaces

echo "[3/8] Ensuring namespaces..."
kubectl create namespace database 2>/dev/null || true
kubectl create namespace monitoring 2>/dev/null || true

# Replace Traefik with NGINX (if not replaced)

echo "[4/8] Installing ingress-nginx (if needed)..."
kubectl get ns ingress-nginx >/dev/null 2>&1 || \
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/cloud/deploy.yaml

# Pull the image

echo "[5/8] Pulling RustyCloud image..."
docker pull "${IMAGE}"

# Apply infra

echo "[6/8] Applying infra..."
kubectl apply -f infra/mongodb
kubectl apply -f infra/redis

# Deploy the application

echo "[7/8] Deploying RustyCloud..."

envsubst < infra/rustycloud/pv.yaml | kubectl apply -f -
kubectl apply -f infra/rustycloud/pvc.yaml

envsubst < infra/rustycloud/secret.yaml | kubectl apply -f -
envsubst < infra/rustycloud/deployment.yaml | kubectl apply -f -
kubectl apply -f infra/rustycloud/service.yaml
envsubst < infra/mongodb/secret.yaml | kubectl apply -f -
envsubst < infra/rustycloud/ingress.yaml | kubectl apply -f -

# Verify

echo "[8/8] Verifying deployment..."

kubectl get nodes
kubectl get pods -A
kubectl get pvc -A
kubectl get svc -A
kubectl get ingress -A

echo "DONE"
