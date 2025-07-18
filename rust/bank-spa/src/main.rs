use axum::{
    http::{header, HeaderValue},
    routing::{get, get_service},
    Router,
};
use std::{
    net::SocketAddr,
    env,
};
use tower_http::{
    compression::CompressionLayer, services::{ServeDir, ServeFile}, set_header::SetResponseHeaderLayer, trace::TraceLayer

};
use anyhow::Result as AnyResult;

#[tracing::instrument]
#[tokio::main]
async fn main() -> AnyResult<()> {
    let tracking_guard = tracking_util::TrackingGuard::init_from_env()?;
    let metrics_handle = tracking_guard.prometheus_handle.clone();
    Ok(serve(
        static_router().route("/metrics", get(|| async move {Ok::<_, std::convert::Infallible>(metrics_handle.render())})),
        get_port()
    ).await)
}


//Set up the router for the static assets
fn static_router() -> Router{
    //gets the directory of the web app to serve and sets fallback to index.html
    let serve_dir = get_service(ServeDir::new("static").append_index_html_on_directories(true).not_found_service(ServeFile::new("static/index.html")));
    //Sets Content Security Policies reqired to run web app (Can Replace with Env var)
    let csp = HeaderValue::from_str(
        &env::var("CSP").unwrap_or(
            "default-src 'self'; \
            connect-src 'self' https://localhost; \
            script-src 'self'; \
            script-src-attr 'self' 'unsafe-inline'; \
            style-src 'self' 'unsafe-inline'; \
            style-src-elem 'self' 'unsafe-inline'; \
            font-src 'self' \
            img-src 'self' data:;".to_string()
        ).replace("\n", "")
    ,
    ).unwrap();
    //Create the actual router with a compression layer and above CSP
    Router::new().nest_service("/", serve_dir.clone()).fallback_service(serve_dir)
    .layer(CompressionLayer::new())
    .layer(SetResponseHeaderLayer::if_not_present(header::CONTENT_SECURITY_POLICY, csp))
    .layer(TraceLayer::new_for_http()) //Tower trace layer for auto instrumentation

}
//Set up the port information
fn get_port() -> u16{
    env::var("SERVER_PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(80)
}
//Serve the router with the port
async fn serve(app: Router, port: u16){
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}