# basic

## api

```sh
curl localhost:8080/
curl localhost:8080/api
curl localhost:8080/db-test

# users
curl localhost:8080/users
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

## docker

```sh
docker run -d --rm \
  --name postgres-test \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=test \
  -p 5432:5432 \
  postgres:latest
```

## sql

```sh
# migration
sqlx migrate run

# setup
docker cp setup.sql postgres-test:/setup.sql
docker exec -it postgres-test psql -U postgres -d postgres -f /setup.sql

# interactive
docker exec -it postgres-test psql -U postgres -d postgres 
INSERT INTO users (name, email) VALUES ('John Doe', 'john.doe@example.com');
\d users
```

## todos

- metrics monitor(logging)
- test
- migration
- cache
- authentication
- swagger
- async task
- vadation
