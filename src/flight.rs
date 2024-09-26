use actix_web::{
    post, web::Json,
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub(super) struct FlightReservationRequest {
    #[schema(example = "Madrid")]
    city_origin: String,
    #[schema(example = "Tokyo")]
    city_destination: String,
    #[schema(example = "09-10-2024")]
    start_date: String,
    #[schema(example = "10-10-2024")]
    end_date: String,
    #[schema(example = 100)]
    seat: u32,
    #[schema(example = "Economy")]
    flight_class: String,
    #[schema(example = "Classic")]
    flight_type: String
}

 #[utoipa::path(
        context_path = "/api",
        responses(
        (status = 201, description = "Create a new flight")
    )
    )]

// #[utoipa::path(
//     responses(
//         (status = 201, description = "Create a new flight")
//     )
// )]
#[post("/flight")]
pub(super) async fn create_flight(data: Json<FlightReservationRequest>) -> impl Responder {
    let data = &data.city_destination;
    let message = format!("Welcome {}", data);
    HttpResponse::Ok().json(message.clone())
}

