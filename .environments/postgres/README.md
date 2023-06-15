# Postgres testing environment

Contains PostgreSQL and PgBouncer instances.

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

Query:

```
docker exec -ti postgres-query-1 /run.sh
```

## Stopping

```
docker-compose down
```
