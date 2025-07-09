#!/bin/bash
set -e

echo "--- Building web Docker image ---"
docker build . -f ./docker/web/Dockerfile -t lilac-web:local

echo "--- Tagging web image for local registry ---"
docker tag lilac-web:local localhost:5001/lilac-web:local

echo "--- Pushing web image to local registry ---"
docker push localhost:5001/lilac-web:local

echo "--- Restarting web deployment in Kubernetes ---"
kubectl rollout restart deployment/lilac-web -n lilac

echo "--- Done! ---"