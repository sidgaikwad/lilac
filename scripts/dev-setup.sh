#!/usr/bin/env sh

set -euxo pipefail

./scripts/kind-setup.sh

docker build . -f ./docker/controlplane/Dockerfile -t lilac-api:local
docker tag lilac-api:local localhost:5001/lilac-api:local
docker push localhost:5001/lilac-api:local

docker build . -f ./docker/web/Dockerfile -t lilac-web:local
docker tag lilac-web:local localhost:5001/lilac-web:local
docker push localhost:5001/lilac-web:local

helm repo add cilium https://helm.cilium.io/
helm repo add community-charts https://community-charts.github.io/helm-charts
helm upgrade --install --debug cilium cilium/cilium --version 1.17.5 --namespace cilium --create-namespace --values - << EOF
k8sServiceHost: lilac-control-plane
k8sServicePort: 6443
kubeProxyReplacement: true
l2announcements:
  enabled: true
encryption:
  enabled: true
  type: wireguard
ingressController:
  enabled: true
  loadbalancerMode: dedicated
  default: true
hubble:
  relay:
    enabled: true
  ui:
    enabled: true
EOF

helm repo add external-secrets https://charts.external-secrets.io
helm install external-secrets external-secrets/external-secrets -n external-secrets --create-namespace

pushd ./k8s/helm-charts/lilac
helm dependency update
popd
helm upgrade --install --wait lilac ./k8s/helm-charts/lilac --namespace lilac --create-namespace