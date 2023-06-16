# Scrape testing environment

Contains agent exporter and scraper instances.

## Starting

```
docker-compose up -d
```

## Running Tests

Redis:

```
docker exec -ti scrape-scrape-static-1 /run.sh
```

## Stopping

```
docker-compose down
```