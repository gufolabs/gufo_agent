# Scrape testing environment

Contains agent exporter and scraper instances.

## Starting

```
docker-compose up -d
```

## Running Tests

Static:

```
docker exec -ti scrape-scrape-static-1 /run.sh
```

DNS:

```
docker exec -ti scrape-scrape-dns-1 /run.sh
```

## Stopping

```
docker-compose down
```
