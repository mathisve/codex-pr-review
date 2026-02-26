use askama::Template;
use axum::response::IntoResponse;

use crate::models::{Booking, Hotel, Room, RoomWithHotel};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub hotels: Vec<Hotel>,
    pub filter_all: bool,
    pub filter_with_pool: bool,
    pub filter_no_pool: bool,
}

#[derive(Template)]
#[template(path = "hotel_detail.html")]
pub struct HotelDetailTemplate {
    pub hotel: Hotel,
    pub rooms: Vec<Room>,
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    pub rooms: Vec<RoomWithHotel>,
    pub city: String,
    pub guests: String,
    pub has_pool: bool,
}

#[derive(Template)]
#[template(path = "room_detail.html")]
pub struct RoomDetailTemplate {
    pub room: RoomWithHotel,
}

#[derive(Template)]
#[template(path = "booking_confirmation.html")]
pub struct BookingTemplate {
    pub booking: Booking,
    pub room: RoomWithHotel,
}

impl IntoResponse for HomeTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

impl IntoResponse for HotelDetailTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

impl IntoResponse for SearchTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

impl IntoResponse for RoomDetailTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

impl IntoResponse for BookingTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}
