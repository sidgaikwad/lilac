# Lilac CLI and Agent

Welcome to the Lilac CLI and Agent documentation. This guide provides instructions for both submitting training jobs and running the agent on your compute nodes.

## Overview

The `lilac_cli` binary serves two primary purposes:

1.  **User CLI**: For data scientists and ML engineers to submit training jobs to the Lilac control plane.
2.  **Agent**: A daemon that runs on compute nodes, connects to the control plane, and executes assigned jobs.

These two modes have separate configurations to ensure clarity and security.

---

## For Users: Submitting Jobs

As a user, your primary interaction with `lilac_cli` will be to configure your API credentials and submit jobs.

### 1. Installation

_(TODO: Installation instructions here)_

### 2. Configuration

To configure the CLI, run the interactive `configure` command:

```bash
lilac_cli configure
```

This will prompt you for the following information:

-   **Lilac API Endpoint**: The URL of the Lilac control plane (e.g., `http://lilac.example.com`).
-   **User API Key**: Your personal API key for authenticating with the Lilac API.

This command creates a configuration file at `~/.lilac/config.toml`.

### 3. Submitting a Job

The `submit` command allows you to submit a pre-built Docker image for execution on the cluster.

```bash
lilac_cli submit
```

The command will interactively guide you through the submission process, asking for:

-   **Job Name**: A descriptive name for your job.
-   **Docker Image URI**: The full URI of your job's Docker image (e.g., `your-registry.com/your-repo/your-image:latest`).
-   **Queue**: The compute queue to submit the job to.
-   **Resource Requirements**: CPU, memory, and GPU requirements for the job.

> **Note:** The CLI no longer builds Docker images. You are responsible for building your image and pushing it to a registry that the Lilac agent can access.

---

## For Administrators: Running the Agent

As an administrator, you will configure and run the Lilac agent on the compute nodes in your cluster.

### 1. Configuration

To configure the agent, run the interactive `agent configure` command:

```bash
lilac_cli agent configure
```

This will prompt you for:

-   **Lilac API Endpoint**: The URL of the Lilac control plane.
-   **Cluster API Key**: The shared secret for authenticating agents with the Lilac cluster.
-   **Private Docker Registry (Optional)**: If your jobs use images from a private registry, you can configure the credentials here.

This command creates a configuration file at `~/.lilac/agent.toml`.

### 2. Starting the Agent

Once configured, you can start the agent daemon with the following command:

```bash
lilac_cli agent start
```

The agent will connect to the control plane, report its available resources, and wait for jobs to be assigned.

---

## Advanced Configuration

Both user and agent configurations can be managed via environment variables, which is ideal for automated deployments (e.g., Kubernetes, systemd).

### Configuration Files

-   **User Config**: `~/.lilac/config.toml`
-   **Agent Config**: `~/.lilac/agent.toml`

### Environment Variables

Environment variables override settings from the configuration files.

| Variable                          | Description                                | Relevant To |
| --------------------------------- | ------------------------------------------ | ----------- |
| `LILAC_API_ENDPOINT`              | The Lilac control plane URL.               | User & Agent|
| `LILAC_USER_API_KEY`              | Your personal user API key.                | User        |
| `LILAC_CLUSTER_API_KEY`           | The shared cluster API key.                | Agent       |
| `LILAC_NODE_ID`                   | A unique ID for the node (optional).       | Agent       |
| `LILAC_PRIVATE_REGISTRY_URL`      | URL of the private Docker registry.        | Agent       |
| `LILAC_PRIVATE_REGISTRY_USERNAME` | Username for the private registry.         | Agent       |
| `LILAC_PRIVATE_REGISTRY_PASSWORD` | Password or token for the private registry.| Agent       |
