# parts

## Example

Suppose `Struct` is defined as follows:

```rust
use parts::Parts;

#[derive(Clone, Parts)]
#[parts_attr(derive(Debug, PartialEq))]
pub struct Struct {
    field1: usize,
    field2: usize,
}
```

Then `StructParts` and `impl`s are derived as follows:

```rust
#[derive(Debug, PartialEq)]
pub struct StructParts {
    pub field1: usize,
    pub field2: usize,
}

impl Struct {
    pub fn into_parts(self) -> StructParts {
        StructParts {
            field1: self.field1,
            field2: self.field2,
        }
    }
}

impl From<Struct> for StructParts {
    fn from(from: Struct) -> StructParts {
        from.into_parts()
    }
}
```
