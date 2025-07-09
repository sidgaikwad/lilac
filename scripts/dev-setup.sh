#!/usr/bin/env sh

set -euxo pipefail

./k8s/kind-setup.sh

docker build . -f ./docker/controlplane/Dockerfile -t lilac-api:local
docker tag lilac-api:local localhost:5001/lilac-api:local
docker push localhost:5001/lilac-api:local

docker build . -f ./docker/web/Dockerfile -t lilac-web:local
docker tag lilac-web:local localhost:5001/lilac-web:local
docker push localhost:5001/lilac-web:local

helm repo add cilium https://helm.cilium.io/
helm upgrade --install --wait --debug cilium cilium/cilium --version 1.17.5 --namespace cilium --create-namespace --values - << EOF
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

helm repo add external-secrets https://charts.external-secrets.io
helm install external-secrets external-secrets/external-secrets -n external-secrets --create-namespace

pushd ./k8s/helm-charts/lilac
helm dependency update
popd
helm upgrade --install --wait lilac ./k8s/helm-charts/lilac --namespace lilac --create-namespace