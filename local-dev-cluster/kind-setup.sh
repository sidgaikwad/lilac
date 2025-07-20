#!/bin/bash
set -e

# The name for our kind cluster
CLUSTER_NAME="lilac-dev"

# Check if kind is installed
if ! [ -x "$(command -v kind)" ]; then
  echo 'Error: kind is not installed.' >&2
  echo 'Please install kind: https://kind.sigs.k8s.io/docs/user/quick-start/#installation' >&2
  exit 1
fi

# Check if a cluster with the same name already exists
if kind get clusters | grep -q "^${CLUSTER_NAME}$"; then
  echo "Cluster '${CLUSTER_NAME}' already exists."
  echo "To delete it, run: kind delete cluster --name ${CLUSTER_NAME}"
  exit 0
fi

echo "Creating kind cluster with name: ${CLUSTER_NAME}"

# Create the cluster
kind create cluster --name "${CLUSTER_NAME}" --config - <<EOF
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
  extraPortMappings:
  - containerPort: 80
    hostPort: 80
    protocol: TCP
  - containerPort: 443
    hostPort: 443
    protocol: TCP
  # Allow NodePort services to be accessed from the host
  - containerPort: 30000
    hostPort: 30000
    listenAddress: "0.0.0.0"
    protocol: TCP
  - containerPort: 30001
    hostPort: 30001
    listenAddress: "0.0.0.0"
    protocol: TCP
  - containerPort: 30002
    hostPort: 30002
    listenAddress: "0.0.0.0"
    protocol: TCP
  - containerPort: 30003
    hostPort: 30003
    listenAddress: "0.0.0.0"
    protocol: TCP
  - containerPort: 30004
    hostPort: 30004
    listenAddress: "0.0.0.0"
    protocol: TCP
  - containerPort: 30005
    hostPort: 30005
    listenAddress: "0.0.0.0"
    protocol: TCP
  - containerPort: 30006
    hostPort: 30006
    listenAddress: "0.0.0.0"
    protocol: TCP
  - containerPort: 30007
    hostPort: 30007
    listenAddress: "0.0.0.0"
    protocol: TCP
  - containerPort: 30008
    hostPort: 30008
    listenAddress: "0.0.0.0"
    protocol: TCP
  - containerPort: 30009
    hostPort: 30009
    listenAddress: "0.0.0.0"
    protocol: TCP
  - containerPort: 30010
    hostPort: 30010
    listenAddress: "0.0.0.0"
    protocol: TCP
EOF

echo "Cluster '${CLUSTER_NAME}' created successfully."
echo "Your kubectl context has been updated to 'kind-${CLUSTER_NAME}'."

# Build and load the jupyter-lilac image into the kind cluster
echo "Building jupyter-lilac:latest docker image..."
docker build -t jupyter-lilac:latest -f local-dev-cluster/Dockerfile .
echo "Loading jupyter-lilac:latest image into cluster..."
kind load docker-image jupyter-lilac:latest --name "${CLUSTER_NAME}"

# Export kubeconfig for the backend to use
echo "Exporting kubeconfig to backend/kubeconfig.yaml..."
kind get kubeconfig --name "${CLUSTER_NAME}" > backend/kubeconfig.yaml

# Create the namespace for the application
echo "Creating namespace 'lilac-dev'..."
kubectl create namespace lilac-dev

echo "You can now use kubectl to interact with your local cluster."