# MySQL testing environment

Contains MySQL instance.

## Starting

```
docker-compose up -d
```

## Running Tests

MySQL:

```
docker exec -ti mysql-mysql-1 /run.sh
```

Query:

```
docker exec -ti mysql-query-1 /run.sh
```

## Stopping

```
docker-compose down
```
