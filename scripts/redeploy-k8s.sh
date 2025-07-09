#!/usr/bin/env sh

set -euxo pipefail

echo "--- Updating Helm repositories ---"
helm repo add cilium https://helm.cilium.io/
helm repo add community-charts https://community-charts.github.io/helm-charts
helm repo update

echo "--- Restarting Cilium ---"
helm upgrade --install --wait --debug cilium cilium/cilium --version 1.17.5 --namespace lilac --create-namespace --values - << EOF
kubeProxyReplacement: true
k8sServiceHost: lilac-control-plane
k8sServicePort: 6443
hostServices:
  enabled: false
externalIPs:
  enabled: true
nodePort:
  enabled: true
hostPort:
  enabled: true
gatewayAPI:
  enabled: true
  service:
    type: NodePort
    nodePorts:
      http: 30080
image:
  pullPolicy: IfNotPresent
ipam:
  mode: kubernetes
hubble:
  enabled: true
  relay:
    enabled: true
EOF

echo "--- Updating Lilac Helm dependencies ---"
pushd ./k8s/helm-charts/lilac
helm dependency update
popd

echo "--- Restarting Lilac ---"
helm upgrade --install --wait lilac ./k8s/helm-charts/lilac --namespace lilac --create-namespace

echo "--- Done! ---"