## I. Goals of the Project

### Project Goal

The goal of the project is to develop and deploy a lightweight distributed file storage platform with secure file management, scalable infrastructure, monitoring, and automated deployment.

The project focuses on:

* simple user authentication and file management
* horizontal scalability using Kubernetes
* centralized persistent file storage
* infrastructure observability and metrics collection
* automated and secure testing and deployment pipeline

### Team Responsibilities

* **Semen Nadutkin** — RustyCloud backend application, Docker containerization, server configuration
* **Magomedgadzhi Ibragimov** — MongoDB, Redis, Prometheus configuration and Kubernetes manifests
* **Stefan Farafonov** — project infrastructure design and documentation, Kubernetes routing configuration
* **Damir Bayazitov** — local GitHub runner deployment, CI/CD pipeline

## II. Methodology

### Infrastructure Overview

#### Core Technologies

* **Backend:** Rust (`axum`)
* **Containerization:** Docker
* **Orchestration:** Kubernetes (`k3s`)
* **Ingress Controller:** NGINX
* **Database:** MongoDB, Redis
* **Monitoring:** Prometheus, Grafana
* **Persistent File Storage:** NFS
* **Deployment Management:** Kustomize
* **CI/CD:** GitHub Actions, Self-hosted Runner
* **Testing:** Pytest

### Infrastructure Design

The infrastructure consists of several interconnected Kubernetes components:

* `NGINX` ingress controller accepts external HTTPS requests
* RustyCloud application pods process client requests
* MongoDB stores persistent user credentials and file metadata
* Redis provides in-memory caching for fast data access
* NFS provides centralized shared file storage for all application replicas
* Prometheus collects infrastructure and application metrics
* Grafana visualizes monitoring and performance data

Internal communication between services is implemented using Kubernetes DNS and `ClusterIP` services.

Persistent components use `PersistentVolume` and `PersistentVolumeClaim` resources for durable storage.

### Request Flow

1. Web clients send requests to [rustycloud.ru](https://rustycloud.ru).
2. NGINX ingress accepts HTTPS traffic and distributes requests between RustyCloud instances.
3. RustyCloud instances authenticate users and process file operations.
4. Metadata is stored in MongoDB and cached in Redis.
5. Uploaded files are stored on the centralized NFS volume.


See **Figure 1: Project architecture diagram** for details.

![alt text](docs/architecture.svg)
**Figure 1: Project architecture diagram**

## III. Development of Solution / Tests as the PoC

### RustyCloud Application

#### Application Functionality

The RustyCloud backend application was implemented in Rust using the `axum` framework.

The application provides:

* user authentication and registration
* file upload, download, and deletion
* session handling using cookies
* HTML template rendering

### HTTP Endpoints

#### Authentication

* `GET /` — login page
* `GET /signup` — signup page
* `POST /login` — user authentication
* `POST /signup` — user registration

#### User Dashboard

* `GET /dashboard` — user dashboard with uploaded files

#### File Operations

* `GET /file` — download a file
* `POST /upload-file` — upload a file
* `POST /delete-file` — delete a file

Authentication is validated using cookies containing user credentials.

See **Figure 2: Web client FSM** for details.

![alt text](docs/web-client-fsm.svg)
**Figure 2: Web client FSM**

### CI/CD Pipeline 

A fully automated CI/CD pipeline was implemented using GitHub Actions and a self-hosted runner.

#### Self-hosted Runner Setup:

* A local Fedora virtual machine was configured as a GitHub Actions runner.
* The runner was registered with the repository and configured to execute pipeline jobs.
* Necessary dependencies (Docker, Python, pytest) were pre-installed on the runner machine.

#### CI/CD Pipeline Stages:

1. **Build** — Docker image of the RustyCloud application is built.
2. **Run** — Virtual Docker network is created, RustyCloud, MongoDB, and Redis containers are started.
3. **Test** — Functional tests are executed against the running application (implemented using the `pytest` framework).
4. **Deploy** — Upon successful tests, the application is automatically deployed to the remote VPS via SSH using the `deploy.sh` script and GitHub Secrets.
5. **Cleanup** — The virtual Docker network and containers are removed from the system.

#### Deployment Automation:

* The `deploy.sh` script pulls the latest Docker image, applies a new configuration to all Kubernetes pods, performs a rolling update using the new Docker image.
* The pipeline uses GitHub Secrets to securely store sensitive information (database credentials, server IP, NFS path).

### Containerization and NGINX Ingress

#### Docker Optimizations:

* A multi-stage Dockerfile was implemented to minimize the final image size.
* The build stage compiles the Rust application, while the runtime stage only includes the compiled binary and necessary dependencies.
* `debian:13-slim` image was used as the base image to reduce the resulting container image size (~20MiB).

#### Kubernetes Manifests:
* Kubernetes `Deployment` and `Service` resources were created to manage the RustyCloud application pods.
* `PersistentVolume` and `PersistentVolumeClaim` were configured for NFS-backed shared storage.
* Kubernetes `Secrets` were used to inject environment variables (database credentials, NFS server address) securely.

#### NGINX Ingress Configuration:

* NGINX Ingress Controller was deployed to handle external HTTP/HTTPS traffic _(replaced default k3s Ingress Controller — `Traefik`)_.
* Ingress rules were defined to route requests from [rustycloud.ru](https://rustycloud.ru) to the RustyCloud service.
* SSL/TLS certificates are used to enable HTTPS encryption _(self-signed)_.

### Kubernetes Infrastructure

The infrastructure was deployed on `k3s` using declarative Kubernetes manifests and Kustomize.

#### RustyCloud Deployment

The RustyCloud application is deployed using Kubernetes `Deployment`, `Service`, `Ingress`, `PersistentVolume`, `PersistentVolumeClaim`, and `Secret` resources.

Implemented configuration includes:

* application deployment using Docker Hub container image
* internal `ClusterIP` service for pod communication
* `NGINX` ingress routing for external access
* NFS-backed shared persistent storage with `ReadWriteMany`
* environment variable injection using Kubernetes Secrets and `envsubst` utility
* shared volume mounting into application containers

This configuration allows application replicas to share centralized file storage and communicate through Kubernetes networking primitives.

#### MongoDB

Implemented using StatefulSet with persistent storage.

* PVC-backed storage
* Kubernetes Secrets for credentials
* automatic initialization scripts via ConfigMap
* internal DNS resolution using headless service

**Limitations:** single replica deployment, no replication or automatic backups

#### Redis

Implemented as an internal caching service.

* ClusterIP service
* lightweight in-memory cache

**Limitations:** no persistence, single replica, no replication or failover

#### Prometheus

Used for metrics collection and monitoring.

* StatefulSet deployment
* persistent storage
* RBAC configuration
* static scrape configuration

**Limitations:** no dynamic service discovery, no Alertmanager integration, single replica deployment

#### Grafana

Used for metrics visualization and dashboard management.

* persistent dashboard storage
* automatic datasource provisioning
* predefined dashboards loaded via ConfigMap

**Limitations:** no external authentication, single instance deployment

### Testing and Proof of Concept

The project includes automated backend tests and deployment automation.

Implemented testing areas:

* HTTP endpoint testing
* authentication flow validation
* file upload and retrieval testing

#### Functional Testing:

* A comprehensive test suite was written using `pytest` and `requests` Python modules.
* The tests verify all critical HTTP endpoints including:
    * User registration (`/signup`) and login (`/login`)
    * Dashboard access (`/dashboard`)
    * File upload (`/upload-file`), download (`/file`), and deletion (`/delete-file`)
    * Authentication and session handling
    * Logout functionality
* Each test validates HTTP status codes, redirect behavior, and response content.

Deployment automation includes:

* image build and push
* deployment script
* Kubernetes rollout deployment process

The infrastructure and application were successfully deployed and tested in a working `k3s` environment.

## IV. Difficulties Faced and New Skills Acquired

### Difficulties

During the project development several technical challenges were encountered:

* configuring communication between Kubernetes services
* debugging DNS resolution in `k3s`
* managing persistent storage for StatefulSets
* integrating NFS with multiple application replicas
* configuring Kubernetes Secrets and ConfigMaps
* debugging container networking and ingress configuration
* integrating monitoring services with Kubernetes workloads

### Skills Acquired

The project provided practical experience with:

* Rust backend development using `axum`
* Docker containerization and multi-stage builds
* Kubernetes resource management
* StatefulSet and Deployment configuration
* Kubernetes networking and DNS
* Kustomize deployment structure
* monitoring stack deployment using Prometheus and Grafana
* CI/CD pipeline integration
* infrastructure debugging and observability

## V. Conclusion

The project demonstrates a functional distributed file storage platform deployed on Kubernetes infrastructure.

The implemented system was successfully deployed on a remote VPS and provides:

* scalable backend deployment
* centralized persistent file storage
* Kubernetes-based orchestration
* infrastructure monitoring and visualization
* persistent storage for stateful services
* automated testing and deployment with CI/CD integration using GitHub Actions and self-hosted runner
* secure HTTPS traffic handling with NGINX and SSL termination
* functional test suite covering authentication and file operations

Future improvements and development vectors:

* WebDAV support
  
* manual testing before release
* REST API support
* automated backup mechanisms
* improved availability of self-hosted GitHub runners
* advanced microservices secrets management _(e.g. AWS Secrets Manager)_

## Links

* Repository: [RustyCloud](https://github.com/SNA-S26/RustyCloud)
* Demonstration: [demo.mp4](https://drive.google.com/file/d/1gz651DXydvx_35uXVcJP09Z2Y1-Dr4UT/view?usp=sharing)
* Deployed application: [rustycloud.ru](https://rustycloud.ru)
* Dockerfile: [Dockerfile](https://github.com/SNA-S26/RustyCloud/blob/main/app/Dockerfile)
* Kubernetes manifests: [infrastructure](https://github.com/SNA-S26/RustyCloud/tree/main/infra)
