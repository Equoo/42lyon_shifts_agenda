# API endpoints

All API calls share the same `/api` endpoint. This means an endpoint described further down as `/users` will have the full path of `/api/users`.

## Type Definitions

- `UserGrade`: represents the grade of a user. Must be one of the following values:
  - Unknown
  - Interested
  - Novice
  - Partner
  - Bartender
  - President
- `Date`: represents a date as a String. Formatted as `YYYY-MM-DD`

## Authentication

### POST `/auth/register`

Request body:
- `username` (String)
- `password` (String)

Response: HTTP 201 (no response body)

### POST `/auth/login`

Request body:
- `username` (String)
- `password` (String)

Response: HTTP 200 with Session Cookie (no response body)

### GET `/auth/me`

Request header:
- Session Cookie

Response body:
- `username` (String)
- `grade` (UserGrade)

JSON Response body:
```json
{
    "username": "mew",
    "grade": "Partner"
}
```


## Users

### GET `/users/{username}`

Request path:
- `username` (String): the `username` for the queried user

Response body:
- `username` (String)
- `grade` (UserGrade)

JSON Response body:
```json
{
    "username": "mew",
    "grade": "Partner"
}
```

## Shifts

### GET `/shifts/{id}`

Request body:
- `id` (Integer): the ID of the shift to query

Response body:
- `id` (Integer)
- `date` (Date)

JSON Response body:
```json
{
    "id": 1,
    "date": "2025-07-03"
}
```

### GET `/shifts/{id}/users`

Request body:
- `id` (Integer): the ID of the shift to query

Response body:
- Array (User)
  - `username` (String)
  - `grade` (UserGrade)

JSON Response body:
```json
[
    {
        "username": "mew",
        "grade": "Partner"
    },
    {
        "username": "panda",
        "grade": "Bartender"
    }
]
```

### GET `/shifts/range?start={date}&end={date}`

Request query params:
- `start` (Date): the range start date
- `end` (Date): the range end date

Response body:
- Array (Shift)
  - `id` (Integer)
  - `date` (Date)

JSON Response body:
```json
[
    {
        "id": 1,
        "date": "2025-07-03"
    },
    {
        "id": 2,
        "date": "2025-07-04"
    }
]
```
