# Postgres testing environment

Contains PostgreSQL and PgBouncer instances.

## Starting

```
docker-compose up -d
```

## Running Tests

Redis:

```
docker exec -ti redis-redis-1 /run.sh
```

## Stopping

```
docker-compose down
```
