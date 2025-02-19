use crate::types;

pub(crate) fn perform_save(state: &types::State) {
    let str = serde_json::to_string(state).unwrap();
    std::fs::write("./state.json", str).unwrap();
    println!("Saved");
}

pub(crate) fn perform_load() -> types::State {
    let file = std::fs::read_to_string("./state.json").unwrap();
    let state: types::State = serde_json::from_str(&file).unwrap();
    return state;
}


