use std::env;
use metrics_exporter_prometheus::PrometheusHandle;
use opentelemetry_otlp::{SpanExporterBuilder, WithExportConfig};
use opentelemetry_sdk::trace::{self};
use pyroscope::pyroscope::PyroscopeAgentReady;
use pyroscope::PyroscopeAgent;
use pyroscope_pprofrs::PprofConfig;
use pyroscope_pprofrs::pprof_backend;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use metrics_exporter_prometheus::PrometheusBuilder;
use opentelemetry_sdk::Resource;
use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider;

pub struct TrackingGuard {
    pub _pyro: Option<pyroscope::PyroscopeAgent<PyroscopeAgentReady>>,
    pub _otel: Option<opentelemetry_sdk::trace::Tracer>,
    pub _prom: Option<PrometheusHandle>
}

impl TrackingGuard {
    pub fn init_from_env() -> anyhow::Result<Self> {
        
         // Prometheus recorder
        let prometheis_handle = PrometheusBuilder::new().install_recorder()?;

        // App/service name
        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "default-app-name".into());

        // Pyroscope setup

        let pyro_agent = Some(
            PyroscopeAgent::builder(
                &env::var("PYRO_ENDPOINT").unwrap_or_else(|_| "http://pyroscope:4040".into()),
                &app_name,
            )
            .backend(pprof_backend(PprofConfig::new().sample_rate(100)))
            .build()?,
        );

        // Tempo (OTLP) tracing
        let tempo_endpoint =
            env::var("TEMPO_ENDPOINT").unwrap_or_else(|_| "http://tempo:4317".into());

        let exporter = SpanExporterBuilder::default().with_tonic().with_endpoint(tempo_endpoint).build()?;
        let provider = trace::TracerProviderBuilder::default()
            .with_resource(Resource::builder().with_attribute(KeyValue::new("service.name", app_name.clone())).build())
            .with_batch_exporter(exporter)
            .build();
        
        let tracer_name = env::var("TRACER_NAME").unwrap_or_else(|_| "default_tracer".into());

        let otel_tracer = provider.tracer(tracer_name);

        // Tracing subscriber
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().json())
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_opentelemetry::layer().with_tracer(otel_tracer.clone()))
            .init();


        Ok(Self {
            _pyro: pyro_agent,
            _otel: Some(otel_tracer),
            _prom: Some(prometheis_handle),
        })
    }
}