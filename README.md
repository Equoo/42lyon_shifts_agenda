# 42lyon_shifts_agenda

A web agenda for the volunteers of the Foyer, the student bar at 42 Lyon: see the week, sign up for day or night shifts, and spot the ones that still need people.
Rust backend (actix-web + MariaDB), React frontend, login with the 42 intranet.
Written in a team of three at 42 Lyon.

## Build and run

```bash
git clone https://github.com/Equoo/42lyon_shifts_agenda
cd 42lyon_shifts_agenda
cat > .env <<EOF
FT_CLIENT_ID=<your 42 app uid>
FT_CLIENT_SECRET=<your 42 app secret>
FT_REDIRECT_URI=http://localhost:3000
EOF
./run_dev.sh            # docker compose build + up
```

The frontend is served on http://localhost:3000, the API on http://localhost:8080 and MariaDB on port 3307.
You need an OAuth application on the 42 intranet whose redirect URI matches `FT_REDIRECT_URI`. The backend refuses to start if one of these variables (or `DATABASE_URL`, set by `docker-compose.yml`) is missing.

The backend runs under `cargo watch`, so it rebuilds on every change. The SQL queries are checked at compile time by `sqlx`, which is why the database container has to be healthy before the backend builds.

## Features

- Login with the 42 intranet (OAuth); the user's login and profile picture are taken from their 42 account
- Week view with a day slot and a night slot for each day, and buttons to move between weeks
- Sign up or leave a shift in one click, until the shift starts (12:00 for day, 20:00 for night)
- Shift colours: complete (enough people and a bartender), almost full, or empty
- Toasts for errors, light and dark theme
- Grades for members (Novice, Member, Bartender, Coordinator, President...); coordinators and the president can add or remove anyone from a shift

## How it works

1. **Login**: the frontend asks the backend for the 42 authorize URL and redirects there. 42 sends the user back with a `code`, which the backend exchanges for a token, then reads `/v2/me`. The user is upserted in the database and stored in a signed session cookie.
2. **Data**: three tables in `init.sql`: `users`, `shifts` (one row per date and slot, with a minimum number of people) and `shifts_user` linking the two. A shift row is created the first time someone signs up for it.
3. **API**: everything lives under `/api` (`/auth/me`, `/shifts/week?start=`, `/shifts/register`, `/shifts/unregister`...). The week endpoint returns the seven days with their registered users in a single query.
4. **Frontend**: a React app (Create React App + Tailwind) that loads the current week, and updates a single shift card from the response after each sign-up.

## What I learned

How OAuth works from start to finish with a real provider, writing a web backend in Rust with compile-time checked SQL, and running a frontend, a backend and a database together with Docker Compose.
