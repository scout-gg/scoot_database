table! {
    building (id) {
        id -> Int4,
        internal_name -> Varchar,
        name -> Int4,
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
    tech_tree_building (building_id) {
        age -> Int2,
        building_id -> Int4,
        enabling_research -> Nullable<Int4>,
        required_building -> Nullable<Int4>,
        required_tech -> Nullable<Int4>,
    }
}

table! {
    tech_tree_tech (tech_id) {
        age -> Int2,
        tech_id -> Int4,
        required_tech -> Nullable<Int4>,
        upper_building -> Int4,
    }
}

table! {
    tech_tree_unit (unit_id) {
        age -> Int2,
        unit_id -> Int4,
        required_tech -> Nullable<Int4>,
        upper_building -> Int4,
        parent_unit -> Nullable<Int4>,
        enabling_research -> Nullable<Int4>,
    }
}

table! {
    technology (id) {
        id -> Int4,
        internal_name -> Varchar,
        name -> Int4,
        building_id -> Nullable<Int4>,
        research_time -> Int4,
        wood_cost -> Int4,
        food_cost -> Int4,
        gold_cost -> Int4,
        stone_cost -> Int4,
    }
}

table! {
    unit (id) {
        id -> Int4,
        internal_name -> Varchar,
        name -> Int4,
        help_text_short -> Int4,
        help_text -> Int4,
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
    }
}

joinable!(building -> help_text (name));
joinable!(civilization -> help_text (name));
joinable!(tech_tree_tech -> building (upper_building));
joinable!(tech_tree_unit -> building (upper_building));
joinable!(technology -> building (building_id));
joinable!(technology -> help_text (name));

allow_tables_to_appear_in_same_query!(
    building,
    civilization,
    help_text,
    tech_tree_building,
    tech_tree_tech,
    tech_tree_unit,
    technology,
    unit,
);
