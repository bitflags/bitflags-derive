#[test]
fn derive_from_str() {
    bitflags! {
        #[derive(FlagsFromStr, PartialEq, Eq, Debug)]
        struct Flags: u8 {
            const A = 1;
            const B = 1 << 1;
            const C = 1 << 2;
        }
    }

    assert_eq!("A | B".parse::<Flags>().unwrap(), Flags::A | Flags::B);
}
