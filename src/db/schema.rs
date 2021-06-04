table! {
    manufacturers (id) {
        id -> Integer,
        name -> Text,
        abbreviation -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    ship_blueprint_classifications (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    ship_blueprint_variant_join_classification (id) {
        id -> Integer,
        ship_blueprint_variant_id -> Integer,
        ship_blueprint_classification_id -> Integer,
    }
}

table! {
    ship_blueprint_variants (id) {
        id -> Integer,
        name -> Text,
        ship_blueprint_id -> Integer,
        description -> Nullable<Text>,
    }
}

table! {
    ship_blueprints (id) {
        id -> Integer,
        name -> Text,
        manufacturer_id -> Integer,
        description -> Nullable<Text>,
    }
}

joinable!(ship_blueprint_variant_join_classification -> ship_blueprint_classifications (ship_blueprint_classification_id));
joinable!(ship_blueprint_variant_join_classification -> ship_blueprint_variants (ship_blueprint_variant_id));
joinable!(ship_blueprint_variants -> ship_blueprints (ship_blueprint_id));
joinable!(ship_blueprints -> manufacturers (manufacturer_id));

allow_tables_to_appear_in_same_query!(
    manufacturers,
    ship_blueprint_classifications,
    ship_blueprint_variant_join_classification,
    ship_blueprint_variants,
    ship_blueprints,
);
