# Serialize Display Adapter Macro Derive

```rust
use serde::Serialize;
use serialize_display_adapter_macro_derive::PrettyJsonSerializeDisplayAdapter;

#[derive(Serialize, PrettyJsonSerializeDisplayAdapter)]
struct Demo<'a> {
    name: &'a str,
    age: u8,
}
fn main() {
    let name = "root";
    let demo = Demo {
        name: name.as_ref(),
        age: 42,
    };
    print!("{}", demo)
}

// Should print:
//
// {
//     "name": "root",
//     "age": 42
// }
```
