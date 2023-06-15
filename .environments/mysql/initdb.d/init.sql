\u metrics
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
CREATE TABLE expenses (
    dept VARCHAR(256),
    region VARCHAR(256),
    value INTEGER
);
INSERT INTO expenses(dept, region, value)
VALUES
    ('it', 'east', 15),
    ('it', 'west', 28),
    ('it', 'east', 7),
    ('sales', 'east', 12),
    ('sales', 'east', 11),
    ('sales', 'west', 7);
