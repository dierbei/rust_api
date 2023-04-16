use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use rust_api::route::{health_check, history_today, miss_short_video, tiktok_beauty, chat};
use rust_api::{get_subscriber, init_subscriber};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("rust_api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api")
                    .service(health_check)
                    .service(tiktok_beauty)
                    .service(history_today)
                    .service(miss_short_video)
                    .service(chat),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
