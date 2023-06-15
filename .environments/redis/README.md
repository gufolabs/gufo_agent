# Redis testing environment

Contains Redis instance.

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

## Sample Database

Check initdb.d/init.sql for the sample database schema
