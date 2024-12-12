use parts::Parts;

#[test]
fn named_fields() {
    #[derive(Clone, Parts)]
    struct Struct {
        field1: usize,
        field2: &'static str,
    }

    let s = Struct {
        field1: 42,
        field2: "test",
    };
    let parts = s.clone().into_parts();

    assert_eq!(parts.field1, s.field1);
    assert_eq!(parts.field2, s.field2);
}
