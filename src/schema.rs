table! {
    building (id) {
        id -> Int4,
        name -> Varchar,
        name_fr -> Varchar,
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
    civilisation (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    civilization (id) {
        id -> Int4,
        name -> Varchar,
        name_fr -> Varchar,
    }
}

table! {
    technology (id) {
        id -> Int4,
        name -> Varchar,
        name_fr -> Varchar,
        building_id -> Nullable<Int4>,
        research_time -> Nullable<Int4>,
        wood_cost -> Int4,
        food_cost -> Int4,
        gold_cost -> Int4,
        stone_cost -> Int4,
    }
}

table! {
    unit (id) {
        id -> Int4,
        name -> Varchar,
        name_fr -> Varchar,
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

joinable!(technology -> building (building_id));

allow_tables_to_appear_in_same_query!(
    building,
    civilisation,
    civilization,
    technology,
    unit,
);
