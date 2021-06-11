CREATE TABLE help_text
(
    id         INT PRIMARY KEY NOT NULL,
    content_en VARCHAR         NOT NULL,
    content_fr VARCHAR,
    content_br VARCHAR,
    content_de VARCHAR,
    content_es VARCHAR,
    content_hi VARCHAR,
    content_it VARCHAR,
    content_jp VARCHAR,
    content_ko VARCHAR,
    content_ms VARCHAR,
    content_mx VARCHAR,
    content_ru VARCHAR,
    content_tr VARCHAR,
    content_tw VARCHAR,
    content_vi VARCHAR,
    content_zh VARCHAR
);

CREATE TABLE civilization
(
    id   INT PRIMARY KEY,
    name INT NOT NULL references help_text (id)
);

CREATE TABLE unit
(
    id                INT PRIMARY KEY,
    unit_type         INT     NOT NULL,
    internal_name     VARCHAR NOT NULL,
    name              INT references help_text (id),
    help_text_short   INT references help_text (id),
    help_text         INT references help_text (id),

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
    internal_name VARCHAR NOT NULL,
    name          INT references help_text (id),
    building_id   INT REFERENCES unit (id),
    research_time INT     NOT NULL,
    wood_cost     INT     NOT NULL,
    food_cost     INT     NOT NULL,
    gold_cost     INT     NOT NULL,
    stone_cost    INT     NOT NULL
);

CREATE TABLE tech_tree_building
(
    id       INT PRIMARY KEY NOT NULL REFERENCES unit (id),
    age               SMALLINT        NOT NULL,
    enabling_research INT REFERENCES technology (id),
    required_building INT REFERENCES unit (id),
    required_tech     INT REFERENCES technology (id)
);

CREATE TABLE tech_tree_tech
(
    id        INT PRIMARY KEY REFERENCES technology (id),
    age            SMALLINT NOT NULL,
    required_tech  INT REFERENCES technology (id),
    upper_building INT      NOT NULL REFERENCES unit (id)
);


CREATE TABLE tech_tree_unit
(
    id           INT PRIMARY KEY REFERENCES unit (id),
    age               SMALLINT NOT NULL,
    required_tech     INT REFERENCES technology (id),
    upper_building    INT      NOT NULL REFERENCES unit (id),
    parent_unit       INT REFERENCES unit (id),
    enabling_research INT REFERENCES technology (id)
);

