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

CREATE TABLE technology
(
    id            INT PRIMARY KEY,
    age           SMALLINT NOT NULL,
    internal_name VARCHAR  NOT NULL,
    name          INT references help_text (id),
    research_time INT      NOT NULL,
    wood_cost     INT      NOT NULL,
    food_cost     INT      NOT NULL,
    gold_cost     INT      NOT NULL,
    stone_cost    INT      NOT NULL,
    is_root BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE unit
(
    id                INT PRIMARY KEY,
    age               SMALLINT NOT NULL,
    unit_type         INT      NOT NULL,
    internal_name     VARCHAR  NOT NULL,
    name              INT references help_text (id),
    help_text_short   INT references help_text (id),
    help_text         INT references help_text (id),
    wood_cost         INT      NOT NULL,
    food_cost         INT      NOT NULL,
    gold_cost         INT      NOT NULL,
    stone_cost        INT      NOT NULL,
    attack            INT      NOT NULL,
    melee_armor       INT      NOT NULL,
    pierce_armor      INT      NOT NULL,
    hit_points        INT      NOT NULL,
    line_of_sight     INT      NOT NULL,
    garrison_capacity INT      NOT NULL,
    is_root BOOLEAN NOT NULL DEFAULT false,
    belongs_to_civ INT REFERENCES civilization(id)
);

CREATE TABLE tech_required_tech
(
    tech          INT PRIMARY KEY REFERENCES technology (id),
    required_tech INT NOT NULL REFERENCES technology (id)
);

CREATE TABLE unit_required_tech
(
    unit          INT PRIMARY KEY REFERENCES unit (id),
    required_tech INT NOT NULL REFERENCES technology (id)
);

CREATE TABLE unit_required_unit
(
    unit          INT PRIMARY KEY REFERENCES unit (id),
    required_unit INT NOT NULL REFERENCES unit (id)
);

CREATE TABLE tech_required_unit
(
    tech INT PRIMARY KEY REFERENCES technology (id),
    required_unit INT NOT NULL REFERENCES unit (id)
);

CREATE TABLE civ_tech
(
    civ_id INT NOT NULL REFERENCES civilization (id),
    tech_id INT NOT NULL REFERENCES technology (id),
    PRIMARY KEY(civ_id, tech_id)
);

CREATE TABLE civ_unit
(
    civ_id INT NOT NULL REFERENCES civilization (id),
    unit_id INT NOT NULL REFERENCES unit (id),
    PRIMARY KEY(civ_id, unit_id)
);



