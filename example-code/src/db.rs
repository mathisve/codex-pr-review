use sqlx::sqlite::SqlitePool;
use std::path::Path;

use crate::models::{Booking, Hotel, Room, RoomWithHotel};

pub async fn init_db(path: impl AsRef<Path>) -> Result<SqlitePool, sqlx::Error> {
    let db_path = path.as_ref();
    let url = format!("sqlite:{}?mode=rwc", db_path.display());
    let pool = SqlitePool::connect(&url).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS hotels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            address TEXT NOT NULL,
            city TEXT NOT NULL,
            country TEXT NOT NULL,
            star_rating INTEGER NOT NULL,
            image_url TEXT
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS rooms (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            hotel_id INTEGER NOT NULL REFERENCES hotels(id),
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            room_type TEXT NOT NULL,
            price_per_night_cents INTEGER NOT NULL,
            max_guests INTEGER NOT NULL,
            image_url TEXT
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bookings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            room_id INTEGER NOT NULL REFERENCES rooms(id),
            guest_name TEXT NOT NULL,
            guest_email TEXT NOT NULL,
            check_in DATE NOT NULL,
            check_out DATE NOT NULL,
            guests INTEGER NOT NULL,
            total_cents INTEGER NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Seed if empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM hotels").fetch_one(&pool).await?;
    if count.0 == 0 {
        seed_data(&pool).await?;
    }

    Ok(pool)
}

async fn seed_data(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO hotels (name, description, address, city, country, star_rating, image_url) VALUES
        ('Grand Plaza Hotel', 'Luxury downtown hotel with stunning city views.', '100 Main Street', 'New York', 'USA', 5, NULL),
        ('Seaside Resort', 'Beachfront resort with private beach and spa.', '50 Ocean Drive', 'Miami', 'USA', 5, NULL),
        ('Mountain Lodge', 'Cozy lodge in the mountains. Perfect for skiing.', '200 Pine Road', 'Aspen', 'USA', 4, NULL)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO rooms (hotel_id, name, description, room_type, price_per_night_cents, max_guests, image_url) VALUES
        (1, 'Deluxe King', 'Spacious room with king bed and city view.', 'deluxe', 29900, 2, NULL),
        (1, 'Executive Suite', 'Luxury suite with living area and skyline view.', 'suite', 49900, 4, NULL),
        (1, 'Standard Double', 'Comfortable double room with all amenities.', 'standard', 18900, 2, NULL),
        (2, 'Ocean View Room', 'Wake up to the sound of the waves.', 'deluxe', 34900, 2, NULL),
        (2, 'Beach Bungalow', 'Private bungalow steps from the beach.', 'bungalow', 59900, 4, NULL),
        (2, 'Garden Room', 'Quiet room with garden view.', 'standard', 22900, 2, NULL),
        (3, 'Mountain View', 'Room with panoramic mountain views.', 'deluxe', 27900, 2, NULL),
        (3, 'Family Suite', 'Two bedrooms, ideal for families.', 'suite', 42900, 6, NULL)
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_hotels(pool: &SqlitePool) -> Result<Vec<Hotel>, sqlx::Error> {
    sqlx::query_as::<_, Hotel>("SELECT id, name, description, address, city, country, star_rating, image_url FROM hotels ORDER BY name")
        .fetch_all(pool)
        .await
}

pub async fn get_hotel(pool: &SqlitePool, id: i64) -> Result<Option<Hotel>, sqlx::Error> {
    sqlx::query_as::<_, Hotel>("SELECT id, name, description, address, city, country, star_rating, image_url FROM hotels WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list_rooms_by_hotel(pool: &SqlitePool, hotel_id: i64) -> Result<Vec<Room>, sqlx::Error> {
    sqlx::query_as::<_, Room>(
        "SELECT id, hotel_id, name, description, room_type, price_per_night_cents, max_guests, image_url FROM rooms WHERE hotel_id = ? ORDER BY price_per_night_cents",
    )
    .bind(hotel_id)
    .fetch_all(pool)
    .await
}

pub async fn search_rooms(
    pool: &SqlitePool,
    city: Option<&str>,
    guests: Option<i64>,
) -> Result<Vec<RoomWithHotel>, sqlx::Error> {
    let rows = sqlx::query_as::<_, RoomWithHotel>(
        r#"
        SELECT r.id, r.hotel_id, r.name, r.description, r.room_type, r.price_per_night_cents, r.max_guests, r.image_url,
               h.name AS hotel_name, h.city AS hotel_city
        FROM rooms r
        JOIN hotels h ON r.hotel_id = h.id
        ORDER BY r.price_per_night_cents
        "#,
    )
    .fetch_all(pool)
    .await?;

    let filtered: Vec<RoomWithHotel> = rows
        .into_iter()
        .filter(|r| {
            let city_ok = city.map(|c| c.is_empty() || r.hotel_city.eq_ignore_ascii_case(c)).unwrap_or(true);
            let guests_ok = guests.map(|g| g <= r.max_guests).unwrap_or(true);
            city_ok && guests_ok
        })
        .collect();

    Ok(filtered)
}

pub async fn get_room(pool: &SqlitePool, id: i64) -> Result<Option<RoomWithHotel>, sqlx::Error> {
    sqlx::query_as::<_, RoomWithHotel>(
        r#"
        SELECT r.id, r.hotel_id, r.name, r.description, r.room_type, r.price_per_night_cents, r.max_guests, r.image_url,
               h.name AS hotel_name, h.city AS hotel_city
        FROM rooms r
        JOIN hotels h ON r.hotel_id = h.id
        WHERE r.id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create_booking(
    pool: &SqlitePool,
    room_id: i64,
    guest_name: &str,
    guest_email: &str,
    check_in: chrono::NaiveDate,
    check_out: chrono::NaiveDate,
    guests: i64,
    total_cents: i64,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO bookings (room_id, guest_name, guest_email, check_in, check_out, guests, total_cents)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(room_id)
    .bind(guest_name)
    .bind(guest_email)
    .bind(check_in)
    .bind(check_out)
    .bind(guests)
    .bind(total_cents)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_booking(pool: &SqlitePool, id: i64) -> Result<Option<Booking>, sqlx::Error> {
    sqlx::query_as::<_, Booking>(
        "SELECT id, room_id, guest_name, guest_email, check_in, check_out, guests, total_cents, created_at FROM bookings WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
