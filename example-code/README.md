# Hotel Booking – Sample Rust Web App

A sample hotel booking website with a Rust backend and server-rendered HTML frontend.

## Stack

- **Backend:** [Axum](https://github.com/tokio-rs/axum) (async web framework)
- **Templates:** [Askama](https://github.com/djc/askama) (type-safe HTML)
- **Database:** SQLite via [SQLx](https://github.com/launchbadge/sqlx)
- **Frontend:** Vanilla CSS and JS, served as static files

## Features

- List hotels and view rooms per hotel
- Search rooms by city and number of guests
- Room detail page with booking form
- Create a booking and see a confirmation page
- Seed data: 3 hotels and 8 rooms (Grand Plaza, Seaside Resort, Mountain Lodge)

## Run locally

```bash
cd example-code
cargo run
```

Then open **http://127.0.0.1:3000**.

- **Database:** A SQLite file `hotel.db` is created in the current directory on first run (or set `DATABASE_URL` to a path like `./my.db`).
- **Static files:** CSS and JS are in `static/` and served at `/static/`.

## Project layout

```
example-code/
├── Cargo.toml
├── src/
│   ├── main.rs       # App setup, routes
│   ├── handlers.rs   # HTTP handlers
│   ├── db.rs         # DB init, queries
│   ├── models.rs     # Hotel, Room, Booking
│   └── templates.rs  # Askama template types
├── templates/        # HTML templates (Askama)
├── static/
│   ├── css/style.css
│   └── js/app.js
└── README.md
```

## Routes

| Method | Path            | Description        |
|--------|-----------------|--------------------|
| GET    | `/`             | Home, list hotels  |
| GET    | `/search`       | Search rooms       |
| GET    | `/hotel/:id`    | Hotel + rooms      |
| GET    | `/room/:id`     | Room + book form   |
| POST   | `/room/:id/book`| Submit booking     |
| GET    | `/booking/:id`  | Booking confirmation |
