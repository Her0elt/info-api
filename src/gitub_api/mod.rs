#[macro_use] extern crate juniper;

use juniper::{FieldResult, Variables, EmptyMutation};

fn main() {
    // Create a context object.

    // Run the executor.
    let (res, _errors) = juniper::execute(
        "query { favoriteEpisode }",
        None,
        &Variables::new(),
        &ctx,
    ).unwrap();

    // Ensure the value matches.
    assert_eq!(
        res.as_object_value().unwrap().get_field_value("favoriteEpisode").unwrap().as_string_value().unwrap(),
        "NEW_HOPE",
    );
}
