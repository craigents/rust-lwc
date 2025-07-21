#[allow(warnings)]
mod bindings;

use bindings::exports::component::wasm_answer::answer::Guest;

struct Component;

impl Guest for Component {
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
