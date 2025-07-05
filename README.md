# Data Pipeline App

## Setup
### Requirements
#### Docker
Install Docker following the instructions at: https://docs.docker.com/engine/install/.

#### Install Kubectl
Install `kubectl` for managing your Kubernetes cluster: https://kubernetes.io/docs/tasks/tools/#kubectl

#### Helm
To install and run Lilac and its dependencies, we rely on Helm charts. You will need to install the Helm CLI: https://helm.sh/docs/intro/install/

#### Kind
For local Kubernetes development, install Kind: https://kind.sigs.k8s.io/docs/user/quick-start/#installation

### Development Environment
To run Lilac on a local Kubernetes cluster, simply run the setup script: `./scripts/dev-setup.sh`. This will set up the following:
- A local kubernetes cluster using Kind
- A local Docker registry that the Kubernetes will pull images from
- Build and push the docker images for Lilac to the local registry
- Install and setup Cilium on the cluster
- Install and setup Postgresql on the cluster
    - Note: for production environments we recommend hosting Postgresql outside of the cluster
- Sets up Lilac on the local Kubernets cluster

After the cluster is deployed, you will need to forward the local ports 8080 and 8081 to be able to reach Lilac from your localhost. Run the following:
```
$ kubectl port-forward -n lilac svc/lilac-web 8080:8080 &
$ kubectl port-forward -n lilac svc/lilac-api 8081:8081 &
```

After this, you should be able to visit `localhost:8080` in your browser and begin interacting with your local version of Lilac.

### Usage
Once the controlplane API and Database are up and running, you can make queries against the API. I recommend installing [HTTPie](https://httpie.io/cli).

```sh
# create a user
$ http :3000/users email=johndoe@example.com password=12345
{
    "id": "bb355a6f-a7c2-4d14-ba59-bbc433e2f4f5"
}

# try to get user without credentials
$ http :3000/users/bb355a6f-a7c2-4d14-ba59-bbc433e2f4f5 
{
    "error": "Missing credentials"
}

# login as user
$ http :3000/auth/login email=johndoe@example.com password=12345
{
    "access_token": "exampleiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImV3aW5ncnlhbjk4QGdtYWlsLmNvbSIsImV4cCI6MTc0MzY2ODM2OX0.KIDR3vFw6Jar-7K9dU_xq5u4SjemW6DFtNWocpuv2os",
    "token_type": "Bearer"
}

# Use JWT token to make request and get user
$ http :3000/users/bb355a6f-a7c2-4d14-ba59-bbc433e2f4f5 Authorization:"Bearer exampleJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImV3aW5ncnlhbjk4QGdtYWlsLmNvbSIsImV4cCI6MTc0MzY2ODM2OX0.KIDR3vFw6Jar-7K9dU_xq5u4SjemW6DFtNWocpuv2os"
{
    "created_at": "2025-04-03T02:19:00.728586Z",
    "email": "johndoe@example.com",
    "id": "bb355a6f-a7c2-4d14-ba59-bbc433e2f4f5"
}

```