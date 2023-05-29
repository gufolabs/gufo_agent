# Postgres testing environment

Contains PostgreSQL and PgBouncer instaces.

## Starting

```
docker-compose up -d
```

## Running Tests

Postgres:

```
docker exec -ti postgres-postgres-1 /run.sh
```

PgBouncer:
```
docker exec -ti postgres-pgbouncer-1 /run.sh
```

## Stopping

```
docker-compose down
```
