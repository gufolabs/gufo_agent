#!/bin/sh
createdb metrics

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname metrics << __EOF__
CREATE TABLE metrics(name VARCHAR(256), value INTEGER);
INSERT INTO metrics(id, name)
VALUES
    ('myapp_read', 12),
    ('myapp_write', 12);
__EOF__