use parts::Parts;

#[test]
fn parts_attr() {
    #[derive(Clone, Parts)]
    #[parts_attr(derive(Debug, PartialEq))]
    struct Struct {
        field: usize,
    }

    assert_eq!(Struct { field: 42 }.into_parts(), StructParts { field: 42 });
}
