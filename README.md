# Lilac

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

Lilac is a distributed computing platform designed to run containerized jobs across a cluster of nodes. It provides a web-based UI for management, a powerful control plane for scheduling, and a lightweight agent for job execution.

## Features

*   **Distributed Job Execution**: Run containerized jobs across a cluster of nodes.
*   **Web-Based UI**: A user-friendly interface for managing the Lilac cluster.
*   **RESTful API**: A comprehensive API for programmatic access to the platform.
*   **Job Queues**: Organize jobs into queues with different priorities.
*   **Resource-Based Scheduling**: The scheduler assigns jobs to nodes based on their resource availability and the job's requirements.
*   **Real-Time Monitoring**: Monitor the status of nodes, queues, and jobs in real-time.
*   **User and API Key Management**: Securely manage user accounts and API keys.

## Architecture

Lilac's architecture is composed of three main components: a Rust-based backend (the control plane), a React-based frontend, and a Rust-based agent.

```mermaid
graph TD
    subgraph "User Interaction"
        A[User] -->|Manages Cluster| B(Frontend UI)
        A -->|Submits Jobs| C(lilac_cli)
    end

    subgraph "Lilac Platform"
        B -->|REST API| D{Backend (Control Plane)}
        C -->|REST API| D

        D -->|Schedules Jobs| E[Agent]
        D -->|Manages State| F[(PostgreSQL DB)]
    end

    subgraph "Compute Nodes"
        E -->|Executes Jobs| G(Docker Container)
    end
```

### Backend (Control Plane)

The backend is a robust Rust application using Axum for its web server and SQLx for asynchronous database interaction with PostgreSQL. It exposes a RESTful API and employs a domain-driven design to structure its business logic.

#### Backend Domains

The backend's logic is organized into several distinct domains:

*   **Auth**: Handles all aspects of user authentication, including username/password login and JWT (JSON Web Token) validation.
*   **Cluster**: Manages the compute clusters, their associated nodes, and API keys. This domain is the primary point of contact for the agents, processing their heartbeats, updating node statuses, and authenticating them.
*   **Queue**: Manages job queues, which allow for the prioritization and organization of jobs. Queues can be configured to target specific clusters.
*   **Training Job**: Oversees the entire lifecycle of a job, from its creation and submission to its final state (succeeded, failed, or canceled).
*   **Scheduler**: The core of Lilac's orchestration. It runs as a continuous background service that:
    *   **Performs Cleanup**: Regularly prunes dead nodes, requeues jobs from failed nodes, and resolves other state inconsistencies.
    *   **Schedules Jobs**: Iterates through the queues in order of priority, finds pending jobs, and assigns them to the first available node in a target cluster that meets the job's resource requirements.

### Frontend

The frontend is a modern single-page application built with React, TypeScript, and Vite. It provides an intuitive and reactive user interface for managing the entire Lilac ecosystem.

Key features include:

*   **Dashboard**: A central hub for monitoring the status of nodes, queues, and jobs.
*   **Cluster Management**: Tools for adding, removing, and inspecting the nodes that form the compute cluster.
*   **Queue Management**: A UI for creating, configuring, and deleting job queues.
*   **Job Management**: A view for submitting new jobs, monitoring their progress, and inspecting their logs and output.
*   **User & API Key Management**: Secure interfaces for managing user accounts and generating API keys for programmatic access.

### Agent

The agent is a lightweight Rust application that runs on every node within a cluster. It's responsible for the hands-on work of job execution.

The agent consists of two parts:

*   **Daemon**: A background service that continuously communicates with the control plane. It sends heartbeats to report the node's health and resource availability, and it executes any jobs assigned to it by the scheduler.
*   **CLI (`lilac_cli`)**: A command-line tool for interacting with Lilac. It's used to start and configure the agent daemon on a node and can also be used to submit jobs to the cluster.

A more detailed README for the agent and `lilac_cli` can be found in the `agent` directory.

## Getting Started

To get Lilac up and running, you'll need to set up the backend, frontend, and at least one agent.

### Prerequisites

*   Rust toolchain
*   Node.js and npm
*   Docker
*   PostgreSQL

### Backend Setup

1.  Clone the repository.
2.  Set up a PostgreSQL database.
3.  Copy `.env.example` to `.env` and configure the `DATABASE_URL` and other settings.
4.  Run the database migrations: `sqlx migrate run`
5.  Start the backend server: `cargo run --bin server`

### Frontend Setup

1.  Navigate to the `frontend` directory.
2.  Install dependencies: `npm install`
3.  Start the development server: `npm run dev`

### Agent Setup

1.  Navigate to the `agent` directory.
2.  Build the agent: `cargo build --release`
3.  Copy the `lilac_agent` binary to each node in your cluster and run it. 

More documentation for the agent exists in the /agent/README.md directory.

## Usage

With all components running, you can access the Lilac UI in your web browser to manage your cluster and jobs. You can also use the `lilac_cli` tool to submit and manage jobs from your terminal.


## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

Lilac is licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).