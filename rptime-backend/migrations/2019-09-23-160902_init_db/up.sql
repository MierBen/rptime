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
    title_ru VARCHAR NOT NULL,
    title_en VARCHAR,
    description_ru VARCHAR NOT NULL,
    description_en VARCHAR,
    flag VARCHAR NOT NULL,
    points INT NOT NULL,
    keys_reward INTEGER [][] NOT NULL,
    keys_condition INTEGER [][] NOT NULL,
    place INT NOT NULL,
    author VARCHAR NOT NULL,
    character INT NOT NULL,
    tags VARCHAR NOT NULL
);

CREATE TABLE team_game (
    id SERIAL PRIMARY KEY,
    team_id INT REFERENCES team_info (id) NOT NULL,
    keys_owned INTEGER [][] NOT NULL,
    points INT NOT NULL
);

CREATE TABLE completed (
    id SERIAL PRIMARY KEY,
    team_id INT REFERENCES team_info (id) NOT NULL,
    task_id INT REFERENCES tasks (id) NOT NULL,
    flag VARCHAR NOT NULL,
    solved BOOL NOT NULL,
    time TIMESTAMP NOT NULL
);
