-- From https://www.dbpilot.io/sql-guides/postgresql/composite-primary-keys-in-postgresql
CREATE TABLE team_members (
    team_id integer,
    person_id integer,
    PRIMARY KEY (team_id, person_id)
);

-- From https://www.postgresql.org/docs/current/tutorial.html
CREATE TABLE weather (
    city varchar(80),
    temp_lo int, -- low temperature
    temp_hi int, -- high temperature
    prcp real, -- precipitation
    date date
);

SELECT
    *
FROM
    weather
WHERE
    city = 'San Francisco'
    AND prcp > 0.0;

