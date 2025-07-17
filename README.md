[![LinkedIn](https://img.shields.io/badge/LinkedIn-Profile-blue?logo=linkedin&logoColor=white)](https://www.linkedin.com/in/roman-zanotelli/)
[![GitHub](https://img.shields.io/badge/GitHub-Profile-green?logo=github&logoColor=white)](https://github.com/Roman-Zanotelli)

# 👁️❤️📈 Grafana Stack Deployment Demo

> A time-boxed observability-focused deployment demo built in 7 days — from scratch — showcasing my ability to rapidly design, implement, and deploy cloud-native infrastructure using the Grafana Labs ecosystem, Rust microservices, and custom Helm charts.

---

## 📸 Screenshots
> Todo

---

## 🔍 About the Project

This repository contains a fictional **banking API platform** with fully integrated **observability**, built and deployed in **7 days** as a self-imposed challenge. The core goal was to demonstrate:

- Hands-on understanding of **Kubernetes deployments**, **Helm packaging**, and **service routing via Envoy**.
- Deep integration of **Grafana Labs tools** (Prometheus, Loki, Tempo, Pyroscope) into a working microservice architecture.
- Practical deployment of real diagnostics on **live Rust services**, all running under HTTPS with metrics, logging, tracing, and profiling.

The project is fully containerized and runs locally on **Minikube**, but was architected with portability toward cloud providers like **Azure Kubernetes Service** in mind.

---

## 🧠 Core Objectives

✅ Design and deploy two Rust microservices simulating a basic banking platform  
✅ Route all external traffic through **Envoy with HTTPS** ingress  
✅ Build and configure **custom Helm charts** for services, Envoy, and PostgreSQL  
✅ Integrate and configure **Grafana, Prometheus, Loki, Tempo, and Pyroscope**  
✅ Wire up **automatic Grafana data source provisioning** (Prometheus, Loki, Tempo, Pyroscope)  
✅ Wrap all services in a unified **umbrella Helm chart** for simple deployment  
✅ Create a **tracking crate** to help standardize observability across services
✅ Create a **jwt-util crate** to help standardize JWT across services  
✅ Ensure **diagnostic feedback** (logs/metrics/traces/profiles) is active and visible in the GUI  
✅ Build a minimal **static web server** for deploying SPA to the browser (SPA frontend is todo)

---

## 🧰 Tech Stack

### Infrastructure
- Kubernetes (Minikube)
- Helm (Custom and official charts)
- Envoy Proxy (HTTPS ingress, Path based Routing)
- PostgreSQL (Ephemeral demo DB with seed script)
- Rust (Axum, SQLx, JWT, tower-http, etc.)
- Helm Umbrella Chart (for orchestration)

### Observability (Grafana Labs Ecosystem)
- Grafana (Dashboard and GUI)
- Prometheus (Metrics)
- Loki (Logs)
- Tempo (Traces)
- Pyroscope (Profiling)
- Beyla (Optional auto-instrumentation)
- Promtail (Log shipping)

---

## 🧪 What Works Right Now

- Services are containerized and deployed via Helm
- Envoy handles HTTPS ingress and JWT validation (ExtAuthZ filter planned)
- Grafana UI is accessible via "https://localhost/grafana" (requires portforwarding 443)
- All data sources (Prometheus, Loki, Pyroscope, Tempo) are **auto-provisioned**
- Sample dashboard (import manually) demonstrates observability metrics
- Minimal static SPA rewired to route cleanly via Envoy
- PostgreSQL is deployed via Helm and seeded on install with a test api_user

---

## 📦 Components

### Rust Microservices
- Auth-Api: Basic sign-up/sign-in API with JWT issuance
- Bank-Api: Account management and transaction simulation
- Bank-SPA: Static web server for deploying frontend SPA
- Metrics, traces, and profiling all embedded via Axum/tower-http + custom middleware

### Static Web Service
- Reused and modified from a prior project
- Minimal but serves as an example of how you would deploy the SPA to the browser
- No froneted files to serve as of yet (Planned to use [LlamaCoder by Together.ai](https://llamacoder.together.ai/) for frontend template)

### Helm Charts
- Custom charts (minimal): PostgreSQL, Envoy, Loki, Prometheus, Auth Service, Api Service, and Static Web Service
- Official charts: Grafana, Promtail, Tempo, Pyroscope
- All wrapped under a unified umbrella chart for fast deploys

---

## 🔐 Security & HTTPS

- Envoy terminates TLS and forwards HTTP to services internally
- JWT validation performed at service levels (proxy level planned with Ext_authz)
- Secrets are provisioned via Helm (demo-only)
- Plans to add Vault/Secret management in future forks

---

## ⚠️ Limitations (By Design)

This project was scoped as a **7-day proof-of-concept**, not a production deployment. Key limitations include:

- No service-to-service TLS (yet)
- No persistent storage (PostgreSQL runs ephemeral)
- No automated CI/CD or GitOps tooling
- No secret rotation or RBAC
- Tracing integration via Tempo is provisioned but **not fully tested**
- Dashboard provisioning was **partially implemented** (auto-load failed, manual import needed)
- Custom tracking library **not fully tested**

---

## 🧠 Reflections & Lessons Learned

This was my first time using many of the observability-related crates in Rust — and my first attempt at integrating the **entire Grafana stack** end-to-end. In one week, I built out:

- Two working services with observability hooks
- Custom and official Helm charts for deployment
- A routed Envoy gateway with HTTPS (Ext_authz planned)
- A running Grafana instance with data sources and diagnostic dashboards

While there’s still a lot I’d like to improve (chart design, application testing, persistent DB, etc.), I’m proud of how much was accomplished from scratch under a short timeline — and I now have a solid foundation for future work in observability, infrastructure automation, and distributed systems diagnostics.

---

## 🔄 Future Plans

- Polish dashboard designs and enable full provisioning on install
- Add persistent storage with backup/restore logic
- Write a full ExtAuthZ microservice for Envoy
- Test and validate database schema and business logic
- Deploy on Azure and run load tests / scaling exercises

---

## 🤖 AI Acknowledgment

Parts of this README and the project scaffolding were assisted by [ChatGPT by OpenAI](https://openai.com/chatgpt).
All architectural decisions, debugging, configuration, and implementation were designed and completed by me.

> ⚖️ **Ethical Use Statement**: I believe in transparent and responsible use of AI. Tools helped accelerate development, but all real-world decisions reflect my own judgment, priorities, and learning.

---

## 📬 Final Thoughts

This project is a snapshot of what I can do under pressure, with new tools, and limited time. It reflects my passion for:

- 🧠 Deep observability
- 🔧 Infrastructure as Code
- 🚀 Hands-on learning and rapid prototyping
- ❤️ Building things that **work** — and learning from what doesn’t (yet)

> The repo will be archived to preserve the original time-boxed demo, with a potential continuation/fork coming soon.

---

Thanks for reading — I hope this demo sparks as much curiosity for you as it did for me!