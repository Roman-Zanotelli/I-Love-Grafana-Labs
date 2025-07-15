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
    pub _pyro: pyroscope::PyroscopeAgent<PyroscopeAgentReady>,
    pub _otel: opentelemetry_sdk::trace::Tracer,
    _loki_task: tracing_loki::BackgroundTask,
    pub prometheus_handle: PrometheusHandle
}

impl TrackingGuard {
    pub fn init_from_env() -> anyhow::Result<Self> {
        
         // Prometheus recorder
        let prometheus_handle = PrometheusBuilder::new().install_recorder()?;

        // App/service name
        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "default-app-name".into());

        // Pyroscope setup

        let _pyro = 
            PyroscopeAgent::builder(
                &env::var("PYRO_ENDPOINT").unwrap_or_else(|_| "http://pyroscope:4040".into()),
                &app_name,
            )
            .backend(pprof_backend(PprofConfig::new().sample_rate(100)))
            .build()?;
        

        // Tempo (OTLP) tracing
        let tempo_endpoint =
            env::var("TEMPO_ENDPOINT").unwrap_or_else(|_| "http://tempo:4317".into());

        let exporter = SpanExporterBuilder::default().with_tonic().with_endpoint(tempo_endpoint).build()?;
        let provider = trace::TracerProviderBuilder::default()
            .with_resource(Resource::builder().with_attribute(KeyValue::new("service.name", app_name.clone())).build())
            .with_batch_exporter(exporter)
            .build();
        
        let tracer_name = env::var("TRACER_NAME").unwrap_or_else(|_| "default_tracer".into());

        let _otel = provider.tracer(tracer_name);

        let loki_endpoint = env::var("LOKI_ENDPOINT").unwrap_or_else(|_| "http://loki:3100".to_string());
        let (loki_layer, _loki_task) = tracing_loki::builder().label("service", app_name)?.build_url(url::Url::parse(&loki_endpoint)?)?;
        // Tracing subscriber
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().json())
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_opentelemetry::layer().with_tracer(_otel.clone()))
            .with(loki_layer) 
            .init();


        Ok(Self {
            _pyro,
            _otel,
            prometheus_handle,
            _loki_task,
        })
    }
}