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
    id   SMALLINT PRIMARY KEY,
    name INT NOT NULL references help_text (id)
);

CREATE TABLE technology
(
    id            SMALLINT PRIMARY KEY,
    age           SMALLINT NOT NULL,
    internal_name VARCHAR  NOT NULL,
    name          INT references help_text (id),
    research_time SMALLINT NOT NULL,
    wood_cost     SMALLINT NOT NULL,
    food_cost     SMALLINT NOT NULL,
    gold_cost     SMALLINT NOT NULL,
    stone_cost    SMALLINT NOT NULL,
    is_root       BOOLEAN  NOT NULL DEFAULT false
);

CREATE TABLE unit
(
    id                SMALLINT PRIMARY KEY,
    age               SMALLINT NOT NULL,
    unit_type         INT      NOT NULL,
    internal_name     VARCHAR  NOT NULL,
    name              INT references help_text (id),
    help_text_short   INT references help_text (id),
    help_text         INT references help_text (id),
    wood_cost         SMALLINT NOT NULL,
    food_cost         SMALLINT NOT NULL,
    gold_cost         SMALLINT NOT NULL,
    stone_cost        SMALLINT NOT NULL,
    attack            SMALLINT NOT NULL,
    melee_armor       SMALLINT NOT NULL,
    pierce_armor      SMALLINT NOT NULL,
    hit_points        SMALLINT NOT NULL,
    line_of_sight     SMALLINT NOT NULL,
    garrison_capacity SMALLINT NOT NULL,
    is_root           BOOLEAN  NOT NULL DEFAULT false,
    belongs_to_civ    SMALLINT REFERENCES civilization (id)
);

CREATE TABLE tech_required_tech
(
    tech          SMALLINT PRIMARY KEY REFERENCES technology (id),
    required_tech SMALLINT NOT NULL REFERENCES technology (id)
);

CREATE TABLE unit_required_tech
(
    unit          SMALLINT PRIMARY KEY REFERENCES unit (id),
    required_tech SMALLINT NOT NULL REFERENCES technology (id)
);

CREATE TABLE unit_required_unit
(
    unit          SMALLINT PRIMARY KEY REFERENCES unit (id),
    required_unit SMALLINT NOT NULL REFERENCES unit (id)
);

CREATE TABLE tech_required_unit
(
    tech          SMALLINT PRIMARY KEY REFERENCES technology (id),
    required_unit SMALLINT NOT NULL REFERENCES unit (id)
);

CREATE TABLE civ_tech
(
    civ_id  SMALLINT NOT NULL REFERENCES civilization (id),
    tech_id SMALLINT NOT NULL REFERENCES technology (id),
    PRIMARY KEY (civ_id, tech_id)
);

CREATE TABLE civ_unit
(
    civ_id  SMALLINT NOT NULL REFERENCES civilization (id),
    unit_id SMALLINT NOT NULL REFERENCES unit (id),
    PRIMARY KEY (civ_id, unit_id)
);



