#[test]
fn derive_display() {
    bitflags! {
        #[derive(FlagsDisplay)]
        struct Flags: u8 {
            const A = 1;
            const B = 1 << 1;
            const C = 1 << 2;
        }
    }

    assert_eq!("A | B", (Flags::A | Flags::B).to_string());
}
