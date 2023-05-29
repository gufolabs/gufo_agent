CREATE DATABASE metrics;
\connect metrics
CREATE TABLE metrics(
    name VARCHAR(256), 
    help VARCHAR(256), 
    value INTEGER
);
INSERT INTO metrics(name, help, value)
VALUES
    ('myapp_read', 'Total reads', 12),
    ('myapp_write', 'Total writes', 28),
    ('myapp_delete', 'Total deletes', 1);
