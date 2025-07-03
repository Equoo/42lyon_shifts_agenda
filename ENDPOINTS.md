# API endpoints

All API calls share the same `/api` endpoint. This means an endpoint described further down as `/users` will have the full path of `/api/users`.

## Users

### Types

- `UserGrade`: represents the grade of a user. Must be one of the following values:
  - Unknown
  - Interested
  - Novice
  - Partner
  - Bartender
  - President

### GET `/users`

Request body:
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

### Types

- `Date`: represents a date as a String. Formatted as `YYYY-MM-DD`

### GET `/shifts`

Request body:
- `shift_id` (Integer): the ID of the shift to query

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

### GET `/shifts/users`

Request body:
- `shift_id` (Integer): the ID of the shift to query

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

### GET `/shifts/range`

Request body:
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
