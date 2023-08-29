# as_base
This crate allows directly accessing fields within a trait object similar to C++ public base classes.
No virtual dispatch is involved, the base object always begins at the same address as the enclosing object.
```rust
use as_base::*;

struct BaseType {
    x: u64,
}

trait MyTrait: AsBase<BaseType> {}

#[derive(AsBase)]
#[repr(C)]
struct Implementor {
    pub base: BaseType,
}
impl MyTrait for Implementor {}

fn main() {
    let x = Implementor {
        base: BaseType { x: 42 },
    };
    let dyn_reference = &x as &dyn MyTrait;
    assert_eq!(dyn_reference.as_base().x, 42)
}
 ```