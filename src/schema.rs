// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    product_base (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        price -> Numeric,
        description -> Nullable<Text>,
        category_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    product_variation (id) {
        id -> Uuid,
        product_base_id -> Nullable<Uuid>,
        #[max_length = 100]
        sku -> Varchar,
        price -> Nullable<Numeric>,
        attributes -> Nullable<Jsonb>,
    }
}

diesel::table! {
    promotions (id) {
        id -> Uuid,
        product_base_id -> Nullable<Uuid>,
        discount_percentage -> Numeric,
        start_date -> Timestamp,
        end_date -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 60]
        password_hash -> Varchar,
    }
}

diesel::joinable!(product_base -> categories (category_id));
diesel::joinable!(product_variation -> product_base (product_base_id));
diesel::joinable!(promotions -> product_base (product_base_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    product_base,
    product_variation,
    promotions,
    users,
);
