use axum::{
    extract::{Path, Query, State},
    response::Redirect,
    Form,
};
use chrono::NaiveDate;
use std::sync::Arc;
use sqlx::SqlitePool;

use crate::db;
use crate::models::BookingForm;
use crate::templates::*;

pub type AppState = Arc<SqlitePool>;

pub async fn home(State(pool): State<AppState>) -> Result<HomeTemplate, axum::http::StatusCode> {
    let hotels = db::list_hotels(&pool).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(HomeTemplate { hotels })
}

pub async fn hotel_detail(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
) -> Result<HotelDetailTemplate, axum::http::StatusCode> {
    let hotel = db::get_hotel(&pool, id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    let rooms = db::list_rooms_by_hotel(&pool, id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(HotelDetailTemplate { hotel, rooms })
}

#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub city: Option<String>,
    pub guests: Option<String>,
}

pub async fn search(
    State(pool): State<AppState>,
    Query(q): Query<SearchQuery>,
) -> Result<SearchTemplate, axum::http::StatusCode> {
    let guests_parsed = q.guests.as_ref().and_then(|s| s.parse::<i64>().ok());
    let rooms = db::search_rooms(&pool, q.city.as_deref(), guests_parsed)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(SearchTemplate {
        rooms,
        city: q.city.unwrap_or_default(),
        guests: q.guests.unwrap_or_default(),
    })
}

pub async fn room_detail(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
) -> Result<RoomDetailTemplate, axum::http::StatusCode> {
    let room = db::get_room(&pool, id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(RoomDetailTemplate { room })
}

pub async fn book_room(
    State(pool): State<AppState>,
    Path(room_id): Path<i64>,
    Form(form): Form<BookingForm>,
) -> Result<Redirect, axum::http::StatusCode> {
    let check_in = NaiveDate::parse_from_str(&form.check_in, "%Y-%m-%d")
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let check_out = NaiveDate::parse_from_str(&form.check_out, "%Y-%m-%d")
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    if check_out <= check_in {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    let guests = form.guests.parse::<i64>().unwrap_or(1).max(1);

    let room = db::get_room(&pool, room_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    let nights = (check_out - check_in).num_days() as i64;
    let total_cents = room.price_per_night_cents * nights;

    let booking_id = db::create_booking(
        &pool,
        room_id,
        &form.guest_name,
        &form.guest_email,
        check_in,
        check_out,
        guests,
        total_cents,
    )
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!("/booking/{}", booking_id)))
}

pub async fn booking_confirmation(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
) -> Result<BookingTemplate, axum::http::StatusCode> {
    let booking = db::get_booking(&pool, id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    let room = db::get_room(&pool, booking.room_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(BookingTemplate { booking, room })
}
