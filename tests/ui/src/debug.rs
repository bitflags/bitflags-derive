#[test]
fn derive_debug() {
    bitflags! {
        #[derive(FlagsDebug)]
        struct Flags: u8 {
            const A = 1;
            const B = 1 << 1;
            const C = 1 << 2;
        }
    }

    assert_eq!("A | B", format!("{:?}", Flags::A | Flags::B));
}
