table! {
    civ_tech (civ_id, tech_id) {
        civ_id -> Int2,
        tech_id -> Int2,
    }
}

table! {
    civ_unit (civ_id, unit_id) {
        civ_id -> Int2,
        unit_id -> Int2,
    }
}

table! {
    civilization (id) {
        id -> Int2,
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
        tech -> Int2,
        required_tech -> Int2,
    }
}

table! {
    tech_required_unit (tech) {
        tech -> Int2,
        required_unit -> Int2,
    }
}

table! {
    technology (id) {
        id -> Int2,
        age -> Int2,
        internal_name -> Varchar,
        name -> Nullable<Int4>,
        research_time -> Int2,
        wood_cost -> Int2,
        food_cost -> Int2,
        gold_cost -> Int2,
        stone_cost -> Int2,
        is_root -> Bool,
    }
}

table! {
    unit (id) {
        id -> Int2,
        age -> Int2,
        unit_type -> Int4,
        internal_name -> Varchar,
        name -> Nullable<Int4>,
        help_text_short -> Nullable<Int4>,
        help_text -> Nullable<Int4>,
        wood_cost -> Int2,
        food_cost -> Int2,
        gold_cost -> Int2,
        stone_cost -> Int2,
        attack -> Int2,
        melee_armor -> Int2,
        pierce_armor -> Int2,
        hit_points -> Int2,
        line_of_sight -> Int2,
        garrison_capacity -> Int2,
        is_root -> Bool,
        belongs_to_civ -> Nullable<Int2>,
    }
}

table! {
    unit_required_tech (unit) {
        unit -> Int2,
        required_tech -> Int2,
    }
}

table! {
    unit_required_unit (unit) {
        unit -> Int2,
        required_unit -> Int2,
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
