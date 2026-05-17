# Usage / Deployment and Verification

## 1. Prerequisites

* Kubernetes cluster (tested on K3s)
* `kubectl` configured and connected to cluster
* Kustomize support enabled

---

## 2. Deployment

Deploy the full stack using Kustomize:

```bash
kubectl apply -k <path-to-kustomization-directory>
```

---

## 3. Access Services

Since all services are `ClusterIP`, access is done via port-forwarding.

### Grafana

```bash
kubectl port-forward svc/grafana-service 3000:3000 -n monitoring
```

Access:
[http://localhost:3000](http://localhost:3000)

---

### Prometheus

```bash
kubectl port-forward -n monitoring pod/prometheus-statefulset-0 9090:9090
```

Access:
[http://localhost:9090](http://localhost:9090)

---

### MongoDB

```bash
kubectl exec -it -n database mongodb-0 -- mongosh -u admin -p admin
```

Basic checks:

```js
show dbs
use appdb
show collections
```

### Redis

```bash
kubectl exec -it -n database <redis-pod-name> -- redis-cli
```

Basic health check:

```bash
ping
```

Expected response: 

```
PONG
```

---

## 4. Basic Verification

Check that all workloads are running:

```bash
kubectl get pods -A
```

Check that storage is bound:

```bash
kubectl get pvc -A
```

Expected result:

* All pods in `Running` state
* All PVCs in `Bound` state

---

## Summary

A successful deployment is confirmed when:

* all Kubernetes workloads are running
* persistent volumes are bound
* Grafana and Prometheus are accessible via port-forward
* services communicate via internal Kubernetes DNS