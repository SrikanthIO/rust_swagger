use std::{error::Error, net::Ipv4Addr};

use actix_web::{middleware::Logger, web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
pub mod flight;
use crate::flight::{create_flight, FlightReservationRequest};

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(paths(api1::hello1))]
    struct ApiDoc1;

    #[derive(OpenApi)]
    #[openapi(paths(api2::hello2))]
    struct ApiDoc2;



    #[derive(OpenApi)]
    #[openapi(
        paths(
            flight::create_flight
        ),
        components(
            schemas(FlightReservationRequest)
        ),
        tags(
            (name = "flight", description = "flight management endpoints.")
        )
    )]
    struct Flight;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(api1::hai1)
                    .service(api1::hello1)
                    .service(flight::create_flight)
                    .service(api2::hello2),
            )
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (
                    Url::new("api1", "/api-docs/openapi1.json"),
                    ApiDoc1::openapi(),
                ),
                (
                    Url::with_primary("api2", "/api-docs/openapi2.json", true),
                    ApiDoc2::openapi(),
                ),(
                    Url::new("flight", "/api-docs/flight.json"),
                    Flight::openapi(),
                ),
            ]))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}

mod api1 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 1", body = String)
        )
    )]
    #[get("/api1/hello")]
    pub(super) async fn hello1() -> String {
        "hello from api 1".to_string()
    }
    #[get("/api1/hai")]
    pub(super) async fn hai1() -> String {
        "hai from api 1".to_string()
    }
}

mod api2 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 2", body = String)
        )
    )]
    #[get("/api2/hello")]
    pub(super) async fn hello2() -> String {
        "hello from api 2".to_string()
    }
}
