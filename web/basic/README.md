# basic

## api

```sh
curl localhost:8080/
curl localhost:8080/api
curl localhost:8080/db-test
curl localhost:8080/users
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
docker cp setup.sql postgres-test:/setup.sql
docker exec -it postgres-test psql -U postgres -d postgres -f /setup.sql

docker exec -it postgres-test psql -U postgres -d postgres 
INSERT INTO users (name, email) VALUES ('John Doe', 'john.doe@example.com');
\d users
```