#!/bin/bash
set -e

echo "--- Building API Docker image ---"
docker build . -f ./docker/controlplane/Dockerfile -t lilac-api:local

echo "--- Tagging API image for local registry ---"
docker tag lilac-api:local localhost:5001/lilac-api:local

echo "--- Pushing API image to local registry ---"
docker push localhost:5001/lilac-api:local

echo "--- Restarting API deployment in Kubernetes ---"
kubectl rollout restart deployment/lilac-api -n lilac

echo "--- Done! ---"