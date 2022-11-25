// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Nullable<Integer>,
        name -> Text,
        day -> Integer,
        starth -> Text,
        endh -> Text,
        isLecture -> Integer,
    }
}
