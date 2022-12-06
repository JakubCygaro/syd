// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Nullable<Integer>,
        name -> Text,
        day -> Text,
        starth -> Text,
        endh -> Text,
        isLecture -> Integer,
    }
}
