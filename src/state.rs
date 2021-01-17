use ::state::Storage;
pub struct State {
    keygrep: Option<String>,
    passphrase: Option<String>,
}

static STATE: Storage<State> = Storage::new();
