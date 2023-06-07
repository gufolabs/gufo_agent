# Postgres testing environment

Contains Redis instance.

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

## Sample Database

Check initdb.d/init.sql for the sample database schema
