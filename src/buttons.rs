use types::*;

fn button_cost(b: Button) -> Option<(Resource, i32)> {
    match b {
        EndTurn => None,
    }
}