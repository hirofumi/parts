use parts::Parts;

#[test]
fn unnamed_fields() {
    #[derive(Clone, Parts)]
    struct Struct(usize, &'static str);

    let s = Struct(42, "test");
    let parts = s.clone().into_parts();

    assert_eq!(parts.0, s.0);
    assert_eq!(parts.1, s.1);
}
