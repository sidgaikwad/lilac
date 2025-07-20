#!/bin/bash
set -a # automatically export all variables
source .env
set +a

export KUBECONFIG=$(kind get kubeconfig --name lilac-dev)
cd backend
LILAC_CONFIG_FILE=./lilac.toml cargo watch -x run