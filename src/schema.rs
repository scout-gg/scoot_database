table! {
    civ_tech (civ_id, tech_id) {
        civ_id -> Int4,
        tech_id -> Int4,
    }
}

table! {
    civ_unit (civ_id, unit_id) {
        civ_id -> Int4,
        unit_id -> Int4,
    }
}

table! {
    civilization (id) {
        id -> Int4,
        name -> Int4,
    }
}

table! {
    help_text (id) {
        id -> Int4,
        content_en -> Varchar,
        content_fr -> Nullable<Varchar>,
        content_br -> Nullable<Varchar>,
        content_de -> Nullable<Varchar>,
        content_es -> Nullable<Varchar>,
        content_hi -> Nullable<Varchar>,
        content_it -> Nullable<Varchar>,
        content_jp -> Nullable<Varchar>,
        content_ko -> Nullable<Varchar>,
        content_ms -> Nullable<Varchar>,
        content_mx -> Nullable<Varchar>,
        content_ru -> Nullable<Varchar>,
        content_tr -> Nullable<Varchar>,
        content_tw -> Nullable<Varchar>,
        content_vi -> Nullable<Varchar>,
        content_zh -> Nullable<Varchar>,
    }
}

table! {
    tech_required_tech (tech) {
        tech -> Int4,
        required_tech -> Int4,
    }
}

table! {
    tech_required_unit (tech) {
        tech -> Int4,
        required_unit -> Int4,
    }
}

table! {
    technology (id) {
        id -> Int4,
        age -> Int2,
        internal_name -> Varchar,
        name -> Nullable<Int4>,
        research_time -> Int4,
        wood_cost -> Int4,
        food_cost -> Int4,
        gold_cost -> Int4,
        stone_cost -> Int4,
        is_root -> Bool,
    }
}

table! {
    unit (id) {
        id -> Int4,
        age -> Int2,
        unit_type -> Int4,
        internal_name -> Varchar,
        name -> Nullable<Int4>,
        help_text_short -> Nullable<Int4>,
        help_text -> Nullable<Int4>,
        wood_cost -> Int4,
        food_cost -> Int4,
        gold_cost -> Int4,
        stone_cost -> Int4,
        attack -> Int4,
        melee_armor -> Int4,
        pierce_armor -> Int4,
        hit_points -> Int4,
        line_of_sight -> Int4,
        garrison_capacity -> Int4,
        is_root -> Bool,
        belongs_to_civ -> Nullable<Int4>,
    }
}

table! {
    unit_required_tech (unit) {
        unit -> Int4,
        required_tech -> Int4,
    }
}

table! {
    unit_required_unit (unit) {
        unit -> Int4,
        required_unit -> Int4,
    }
}

joinable!(civ_tech -> civilization (civ_id));
joinable!(civ_tech -> technology (tech_id));
joinable!(civ_unit -> civilization (civ_id));
joinable!(civ_unit -> unit (unit_id));
joinable!(civilization -> help_text (name));
joinable!(tech_required_unit -> technology (tech));
joinable!(tech_required_unit -> unit (required_unit));
joinable!(technology -> help_text (name));
joinable!(unit -> civilization (belongs_to_civ));
joinable!(unit_required_tech -> technology (required_tech));
joinable!(unit_required_tech -> unit (unit));

allow_tables_to_appear_in_same_query!(
    civ_tech,
    civ_unit,
    civilization,
    help_text,
    tech_required_tech,
    tech_required_unit,
    technology,
    unit,
    unit_required_tech,
    unit_required_unit,
);
