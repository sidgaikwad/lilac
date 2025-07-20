#!/bin/bash
cd backend

set -a # automatically export all variables
source .env
set +a

KUBECONFIG=$(kind get kubeconfig --name lilac-dev) cargo run --bin server