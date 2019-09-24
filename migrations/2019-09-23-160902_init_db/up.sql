CREATE TABLE team_info (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    country VARCHAR,
    university VARCHAR,
    token VARCHAR NOT NULL
);

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    task_name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    flag VARCHAR NOT NULL,
    points VARCHAR NOT NULL,
    keys_reward json NOT NULL,
    keys_condition json NOT NULL,
    coords json NOT NULL
);

CREATE TABLE team_game (
    id SERIAL PRIMARY KEY,
    team_id INT REFERENCES team_info (id),
    keys_owned json NOT NULL,
    points INT NOT NULL
);

CREATE TABLE completed (
    id SERIAL PRIMARY KEY,
    team_id INT REFERENCES team_info (id),
    task_id INT REFERENCES tasks (id),
    time TIMESTAMP NOT NULL
);