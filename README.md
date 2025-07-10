# 👁️ ❤️ Grafana
I Love Grafana! So I made a demo project to show off using their solutions.

## About Me
I'm a passionate developer with a deep interest in systems architecture, cloud-native technologies, and observability-driven development. My journey began over a decade ago, programming in Java at the age of 14 to build and orchestrate game systems — a passion that quickly evolved into exploring Kubernetes, infrastructure automation, and scalable backend solutions.

I’ve since worked to develop my own business, designing and deploying cost-optimized, secure systems from the ground up. With experience in Helm, Kubernetes, TalosOS, ArgoCD, Envoy, Redis, Jenkins, PostgreSQL, gRPC, and more, I focus on building efficient, profitable, and resilient systems that are production-ready and resource-aware.

I believe observability is a core pillar of modern software, and Grafana’s tools have played a pivotal role in my personal and professional projects — from monitoring self-hosted infrastructure to fine-tuning distributed services. I'm particularly drawn to the open-source ethos and thoughtful design behind Grafana's ecosystem and strive to build solutions that reflect those same values.

Whether working in Rust, Go, or Java, I bring a modern, adaptable, and low-level-capable mindset — from edge compute to backend orchestration. This project serves as both a technical showcase and a stepping stone in my continued growth in cloud development, observability, and open-source tooling.

[![LinkedIn](https://img.shields.io/badge/LinkedIn-Profile-blue?logo=linkedin&logoColor=white)](https://www.linkedin.com/in/roman-zanotelli/)
[![GitHub](https://img.shields.io/badge/GitHub-Profile-green?logo=github&logoColor=white)](https://github.com/Roman-Zanotelli)

---

## 🎯 Project Overview

This project is a **demo deployment of a custom business logic API** using Kubernetes and Helm. The API will be exposed via a **RESTful HTTPS interface**, secured with **Envoy** as the ingress gateway. The primary focus is to demonstrate the full power of the **Grafana observability ecosystem**, all running seamlessly in **Minikube** or a **public Kubernetes platform**, deployed through a **single umbrella Helm chart**.

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
- [Minikube](https://minikube.sigs.k8s.io/docs/)
- [Envoy Proxy](https://www.envoyproxy.io/)

### Observability Stack (by Grafana Labs)
- [Grafana Dashboard](https://grafana.com/)
- [Prometheus](https://prometheus.io/)
- [Loki (Logs)](https://grafana.com/oss/loki/)
- [Pyroscope (Profiling)](https://grafana.com/oss/pyroscope/)
- [Tempo (Tracing)](https://grafana.com/oss/tempo/)
- [Faro (Frontend Monitoring)](https://grafana.com/oss/faro/) *(optional)*

### Programming Language
- [Rust](https://www.rust-lang.org/)
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

While I'm still new to some of these tools, this project highlights my:
- ❤️ Passion for Grafana and observability.
- 🔄 Adaptability and eagerness to learn modern cloud-native tooling.
- 🛠️ Practical experience integrating complex systems in Kubernetes.

> This demo isn’t just about functionality—it’s about **showing the mindset and understanding** needed to deploy cloud applications with grafana's industry tested solutions.

---

## 🤖 AI Assistance Acknowledgment

Parts of this README and project scaffolding were created with assistance from [ChatGPT by OpenAI](https://openai.com/chatgpt) to help organize ideas, improve clarity, and accelerate development.

Sections of the front-end were created with assistance from [LlamaCoder by Together.ai](https://llamacoder.together.ai/) in conjuction with [ChatGPT](https://openai.com/chatgpt) to help accelerate application development.

> ⚖️ **Ethical Use Statement**: I believe in responsible and transparent use of AI tools. While AI helped shape structure and language, all architectural decisions, implementation, and final content reflect my own understanding, judgment, and intent.

