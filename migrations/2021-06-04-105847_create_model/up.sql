CREATE TABLE civilization
(
    id      INT PRIMARY KEY,
    name    VARCHAR NOT NULL,
    name_fr VARCHAR NOT NULL
);

CREATE TABLE unit
(
    id                INT PRIMARY KEY,
    name              VARCHAR NOT NULL,
    name_fr           VARCHAR NOT NULL,
    wood_cost         INT     NOT NULL,
    food_cost         INT     NOT NULL,
    gold_cost         INT     NOT NULL,
    stone_cost        INT     NOT NULL,
    attack            INT     NOT NULL,
    melee_armor       INT     NOT NULL,
    pierce_armor      INT     NOT NULL,
    hit_points        INT     NOT NULL,
    line_of_sight     INT     NOT NULL,
    garrison_capacity INT     NOT NULL
);

CREATE TABLE building
(
    id                INT PRIMARY KEY,
    name              VARCHAR NOT NULL,
    name_fr           VARCHAR NOT NULL,
    wood_cost         INT     NOT NULL,
    food_cost         INT     NOT NULL,
    gold_cost         INT     NOT NULL,
    stone_cost        INT     NOT NULL,
    attack            INT     NOT NULL,
    melee_armor       INT     NOT NULL,
    pierce_armor      INT     NOT NULL,
    hit_points        INT     NOT NULL,
    line_of_sight     INT     NOT NULL,
    garrison_capacity INT     NOT NULL
);

CREATE TABLE technology
(
    id            INT PRIMARY KEY,
    name          VARCHAR NOT NULL,
    name_fr       VARCHAR NOT NULL,
    building_id   INT REFERENCES building (id),
    research_time INT,
    wood_cost     INT     NOT NULL,
    food_cost     INT     NOT NULL,
    gold_cost     INT     NOT NULL,
    stone_cost    INT     NOT NULL
);
