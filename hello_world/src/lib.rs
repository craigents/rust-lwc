#[allow(warnings)]
mod bindings;

use bindings::exports::component::hello_world::greeter::Guest;

struct Component;

impl Guest for Component {
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
