# RustyCloud Kubernetes Stack (k3s)

This project contains Kubernetes manifests deployed on K3s using Kustomize.
It implements a basic infrastructure consisting of databases and monitoring components.

---

# I. Goal / Tasks of the Project

## Project Goal

The goal of this part of the project is to deploy and configure a working Kubernetes-based infrastructure using declarative YAML manifests.

The implemented system includes:

* MongoDB StatefulSet with persistent storage
* Redis Deployment as in-memory cache
* Prometheus StatefulSet for metrics collection
* Grafana Deployment for visualization
* Kustomize-based deployment structure

---

## Responsibilities (relevant to this part)

### Infrastructure / Kubernetes Configuration Role

* Writing Kubernetes manifests for all services
* Configuring Services (ClusterIP / headless)
* Configuring StatefulSets and Deployments
* Managing PersistentVolumeClaims (PVC)
* Configuring ConfigMaps and Secrets
* Structuring deployment using Kustomize

---

# II. Execution Plan / Methodology

## Deployment Structure

The system is deployed using Kustomize:

* namespaces separation:

  * `database`
  * `monitoring`

* workload types:

  * StatefulSet (MongoDB, Prometheus)
  * Deployment (Redis, Grafana)

---

## Service Networking Model

* Internal communication is handled via Kubernetes DNS
* Services are exposed using ClusterIP (internal only)
* Prometheus uses static scraping configuration
* Grafana connects to Prometheus via internal service DNS

---

## Storage Design

* MongoDB uses PVC (local-path storage class)
* Prometheus uses PVC for TSDB data retention
* Grafana uses PVC for dashboard and metadata storage

---

# III. Development / PoC / Testing

## MongoDB Configuration

### Implementation

### Implementation

* StatefulSet with one replica
* Persistent storage via PVC
* Credentials stored in Kubernetes Secret
* Headless service used for internal DNS resolution
* Initialization script injected via ConfigMap (`/docker-entrypoint-initdb.d`)
* Automatic database bootstrap on first startup (schema + index creation)

### Issues / Limitations

* Uses single replica (no replication or HA)
* Depends on local-path storage (node-bound data)
* No automated backup mechanism
* No network-level security policies

---

## Redis Configuration

### Implementation

* Deployment with single replica
* ClusterIP service for internal access
* Used as in-memory cache only

### Issues / Limitations

* No persistence layer configured
* No authentication enabled by default
* No memory eviction policy explicitly defined
* Single point of failure (no replication)

---

## Prometheus Configuration

### Implementation

* StatefulSet with persistent storage
* ConfigMap-based static configuration
* RBAC enabled for Kubernetes API access
* Headless service for StatefulSet identity

### Issues / Limitations

* Uses static scrape configuration (no dynamic service discovery)
* Limited scalability (single replica)
* Short retention period (12h)
* No Alertmanager integration
* Dependent on manual configuration for new targets

---

## Grafana Configuration

### Implementation

* Deployment with persistent storage (PVC)
* Datasource provisioning via ConfigMap
* Preconfigured Prometheus datasource
* ClusterIP service for internal access
* Dashboard provider configuration (file-based provisioning)
* Predefined JSON dashboards mounted into Grafana at runtime
* Dashboards automatically loaded from `/var/lib/grafana/dashboards`

### Issues / Limitations

* No external authentication layer configured
* Uses local persistent storage (no HA)
* Single instance (no replication)
* No centralized secrets management for admin credentials

---

# IV. Difficulties and New Skills Acquired

## Difficulties

* Managing correct Kubernetes networking (ClusterIP vs headless services)
* Ensuring correct DNS resolution between services
* Configuring persistent storage in a single-node k3s environment
* Aligning ConfigMap-based configurations with application expectations
* Debugging StatefulSet and PVC binding behavior

## Skills Acquired

* Kubernetes resource design (StatefulSet, Deployment, Service, PVC)
* Service discovery using Kubernetes DNS
* ConfigMap and Secret management
* Basic observability stack configuration (Prometheus + Grafana)
* Kustomize-based multi-resource deployment structure

---

# V. Conclusion

This part of the project demonstrates a functional Kubernetes-based infrastructure deployed using declarative manifests.

The system successfully implements:

* persistent storage for stateful services
* internal service networking via Kubernetes DNS
* metrics collection and visualization pipeline
* basic separation of infrastructure layers

However, the current implementation is limited by:

* single-node storage dependency
* absence of high availability
* static monitoring configuration
* minimal security hardening