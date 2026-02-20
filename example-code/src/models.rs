use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Hotel {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub star_rating: i64,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Room {
    pub id: i64,
    pub hotel_id: i64,
    pub name: String,
    pub description: String,
    pub room_type: String,
    pub price_per_night_cents: i64,
    pub max_guests: i64,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RoomWithHotel {
    pub id: i64,
    pub hotel_id: i64,
    pub name: String,
    pub description: String,
    pub room_type: String,
    pub price_per_night_cents: i64,
    pub max_guests: i64,
    pub image_url: Option<String>,
    pub hotel_name: String,
    pub hotel_city: String,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Booking {
    pub id: i64,
    pub room_id: i64,
    pub guest_name: String,
    pub guest_email: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub guests: i64,
    pub total_cents: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct BookingForm {
    pub guest_name: String,
    pub guest_email: String,
    pub check_in: String,
    pub check_out: String,
    pub guests: String,
}

impl Hotel {
    pub fn stars_display(&self) -> String {
        "★".repeat(self.star_rating as usize)
    }
}

impl Room {
    pub fn price_display(&self) -> String {
        format!("${}.{:02}", self.price_per_night_cents / 100, self.price_per_night_cents % 100)
    }
}

impl RoomWithHotel {
    pub fn price_display(&self) -> String {
        format!("${}.{:02}", self.price_per_night_cents / 100, self.price_per_night_cents % 100)
    }
}

impl Booking {
    pub fn total_display(&self) -> String {
        format!("${}.{:02}", self.total_cents / 100, self.total_cents % 100)
    }
}
