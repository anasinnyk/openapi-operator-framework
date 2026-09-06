use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use cloudflare_provider::run_controller;
use kube::Client;

#[get("/")]
async fn ok() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    match Client::try_default().await {
        Ok(client) => {
            tokio::spawn(async move {
                if let Err(error) = run_controller(client).await {
                    tracing::error!(?error, "Cloudflare controller stopped");
                }
            });
        }
        Err(error) => {
            tracing::warn!(
                ?error,
                "Kubernetes client is not configured; starting HTTP server without controller"
            );
        }
    }

    HttpServer::new(|| App::new().service(ok))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
