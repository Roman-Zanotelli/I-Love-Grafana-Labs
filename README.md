[![LinkedIn](https://img.shields.io/badge/LinkedIn-Profile-blue?logo=linkedin&logoColor=white)](https://www.linkedin.com/in/roman-zanotelli/)
[![GitHub](https://img.shields.io/badge/GitHub-Profile-green?logo=github&logoColor=white)](https://github.com/Roman-Zanotelli)
# 👁️-❤️-📈📉
I Love Grafana! So I made a demo project to show off using their solutions.

### Me:

I'm a passionate developer with a deep interest in systems architecture, cloud-native technologies, and security/observability-driven development. My journey began over a decade ago, programming in Java at the age of 14 to build and orchestrate game systems — a passion that quickly evolved into exploring Kubernetes, infrastructure automation, and scalable backend solutions.

I’ve since worked to develop my own business, designing and deploying cost-optimized, secure systems from the ground up. With experience in Helm, Kubernetes, TalosOS, ArgoCD, Envoy, Redis, Jenkins, PostgreSQL, gRPC, and more, I focus on building efficient, profitable, and resilient systems that are production-ready and resource-aware.

I believe observability is a core pillar of modern software, and Grafana’s tools have played a pivotal role in my personal and professional projects — from monitoring self-hosted infrastructure to fine-tuning distributed services. I'm particularly drawn to the open-source ethos and thoughtful design behind Grafana's ecosystem and strive to build solutions that reflect those same values.

Whether working in Rust, Go, or Java, I bring a modern, adaptable, and low-level-capable mindset — from edge compute to physical machine orchestration. I earned my KCNA (Kubernetes and Cloud Native Associate) certification on 2024-01-27 and am currently pursuing Microsoft Azure certifications to broaden my understanding of public cloud infrastructure and enterprise-grade deployments.

By combining hands-on experience in private, self-hosted systems with ongoing education in public cloud technologies, I aim to maintain a holistic grasp of modern infrastructure — across private, public, and hybrid deployments; With Azure, a leading provider trusted by Fortune 500 companies and government sectors representing a crucial piece of that puzzle. As I deepen my exposure to enterprise tooling, I plan to continue towards becoming a Kubernetes Certified Architect.

This project serves as both a technical showcase and a marker/milestone in my continued growth in cloud development, observability, and open-source tooling — with the ultimate goal of designing and building beautifully impactful, secure, reliable systems at scale.

---

## 🎯 Project Overview

This project is a demo of my ability to **design, implement, and deploy custom buisness needs in a cloud enviroment** using Kubernetes and Helm. The API will be exposed via a **RESTful HTTPS interface**, secured with **Envoy** as the gateway. The primary focus is to demonstrate the full power of the **Grafana observability ecosystem**, a personal of mine favorite for a long time, all seamlessly running in **Minikube** or a **public Kubernetes platform** (Azure), deployed through a **single umbrella Helm chart**.

Because this is a demo project all content will be within this single repo, normally I would be seperated into distinct repos for each service/chart through a dedicated buisness account but prefer to keep it together for my personal account.

There will also be many missing features absent that would otherwise be best practice for production (Service-to-Service TLS, Persistent Storage, Secret Management/Rotation, CI/CD, etc); I do plan to update this demo in the future with said additions but for now the focus is again on the Grafana Stack.

This project was planned to be completed in ~5 days (Jul 10) 2 planning, 3 for implementation, this is meant to challenge myself to prove what Im capable of from (relative) scratch under limited time. 
> **EDIT:** Ive given myself an extension on the project by 2 days for a total of ~7, quality over speed, also I got my wisdom teeth removed :^}

## 🧠 Key Goals

- ✅ **Deploy a simple example banking API** with secure HTTPS routing.
- ✅ **Use Envoy** for ingress and TLS termination.
- ✅ **Showcase Grafana’s observability stack** in a real-world Kubernetes deployment.
- ✅ Deploy everything via a **unified Helm umbrella chart** for easy reproducibility.
- ✅ Highlight my passion for Grafana tooling and observability best practices.
- ✅ Demonstrate adaptability, enthusiasm, and hands-on exposure to modern cloud-native tools.

## 🛠️ Technologies Used

### Infrastructure & Deployment
- [Kubernetes](https://kubernetes.io/)
- [Helm](https://helm.sh/)
- [Minikube](https://minikube.sigs.k8s.io/docs/) *(or k8s provider)*
- [Envoy Proxy](https://www.envoyproxy.io/)
- [PostgreSQL](https://www.postgresql.org/)

### Observability Stack (Grafana Labs + Prometheus)
- [Grafana Dashboard (Visualization)](https://grafana.com/grafana/)
- [Prometheus (Metrics)](https://prometheus.io/) [[Additional Info]](https://grafana.com/oss/prometheus/)
- [Grafana Loki (Logs)](https://grafana.com/oss/loki/)
- [Grafana Pyroscope (Profiling)](https://grafana.com/oss/pyroscope/)
- [Grafana Tempo (Tracing)](https://grafana.com/oss/tempo/)
- [Grafana Beyla (eBPF Auto-Instrumentation)](https://grafana.com/oss/beyla-ebpf/)
- [Grafana Faro (Frontend Monitoring)](https://grafana.com/oss/faro/) *(optional, implement if time permits)*

### Programming Language & Crates
- [Rust](https://www.rust-lang.org/)
- [Jsonwebtoken (JWT)](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/index.html)
- [Axum (REST-API/Web)](https://docs.rs/axum/latest/axum/)
- [Sqlx (SQL)](https://docs.rs/sqlx/latest/sqlx/)
---

## 💰 Demo Application: Example Banking API

A lightweight RESTful API simulating basic banking operations:
- Endpoints for creating accounts, viewing balances, transferring funds, etc.
- Instrumented for metrics, logs, traces, and profiling hooks.
- Secured with HTTPS via Envoy.

## 🌈 What This Demo Shows

- 🔍 End-to-end observability from API request to trace/log/metric/profile.
- 📦 Seamless deployment and configuration using Helm.
- 📊 Rich, preconfigured dashboards in Grafana.
- 💡 How an adaptable, enthusiastic developer integrates a full observability stack from scratch.

---

## 💬 Final Thoughts

While I'm still learning some of these tools, this project highlights my:
- ❤️ Passion for Grafana and observability.
- 🔄 Adaptability and eagerness to learn modern cloud-native tooling.
- 🛠️ Practical experience integrating complex systems in Kubernetes.

> ! This demo isn’t just about functionality—it’s about **showing the mindset and understanding** needed to deploy cloud applications with grafana's (and other) industry tested solutions.

---


# 🔍 Getting Detailed...
> My Favorite Part :)

Core objectives:
- Sign-Up/In Account
- Send/Recieve Money
- Saved Account Management
- Queriable Transaction History
- Extensive Observability Stack (More on this Later)

Microservices:
- Static SPA Web Server
- User Auth API
- Bank API
- Envoy
- Postgre (could be replaced with another solution)
- Grafana Dashboard
- Prometheus
- Grafana Loki
- Grafana Pyroscope
- Grafana Tempo

Storage is going to be skipped in this demo, same for CI/CD, Secret Rotation, and Rate Limiting; With the focus again being on the observability stack allowing these improvements to be gauged and implemented in the future.

## Verification
Verification will be done at a proxy level through envoy's Ext_Authz filter and at an application level inside of the Bank API to ensure Security at all levels.

Both verify very similar to one another with the exception being the JWT Authentication will prevalidate the claim as its sole purpose, while the Bank API will run this in a seperate task while assuming it to be correct (still verifying before doing any final actions such as changing balances) using transactions to cancel/rollback changes in the event the JWT is for some reason invalid.

JWT Secret is going to be deployed by helm in this demo, in production some sort of secret management solutions would be used. (such as vault)

## HTTPS
Https will be handled by Envoy and then sent to the services as Http, Using Istio or some other solution would be used to encrypt sevrice to service communication in production.

For demo purposes the cert will be self signed and be deployed by helm, Again in production I would use some sort of secret management.

## Proxy
Envoy routes all trafic to their proper microservice while verifying their JWTs

All microservices should be unreachable except through the proxy

Routes:
- / -> static-web-service
- /signin -> auth-api-service
- /signup -> auth-api-service
- /account -> bank-api-service
- /account/send -> bank-api-service
- /account/recieve -> bank-api-service
- /account/transactions -> bank-api-service
- /account/contacts -> bank-api-service
- /grafana -> grafana-service
> These may change as the API is developed

## PostgreSQL
Simple SQL Setup for the purposes of the demo
- Used for login and buisness data
- Connect to observability stack

Grafana Beyla will probably simplify Postgre Observability Set Up

> Todo: Outline DB/Table Structure For Project

> Todo: More Detailed DB Explination

## Grafana Dashboard
Proxy aware routing (serve from path needs to match envoy routing)
Envoy recieves it as https but is propgated back to the dashboard as http
> Todo: Add default data source
> Todo: Add default dashboards
> Todo: Create fine-tuned/asthetic dashboards for demo

## Prometheus
Custom Prometheus Chart
Using metrics, metrics-exporter-prometheus, and tower-http crate for metrucs auto-instrumentation with axum
Scapping envoy metrics on dedicated service
> Todo: Add postgres_exporter sidecar to export postgre metrics

## Loki
> Todo

## Tempo
> Todo

## Pyroscope
> Todo

# 🤖 AI Assistance Acknowledgment

Parts of this README and project scaffolding were created with assistance from [ChatGPT by OpenAI](https://openai.com/chatgpt) to help organize ideas, improve clarity, and accelerate development.

Sections of the front-end were created with assistance from [LlamaCoder by Together.ai](https://llamacoder.together.ai/) in conjuction with [ChatGPT](https://openai.com/chatgpt) to help accelerate development.

> ⚖️ **Ethical Use Statement**: I believe in responsible and transparent use of AI tools. While AI helped shape structure and language, all architectural decisions, implementation, and final content reflect my own understanding, judgment, and intent.