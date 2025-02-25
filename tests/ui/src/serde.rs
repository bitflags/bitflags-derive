use serde_test::{assert_tokens, Configure, Token::*};

#[test]
fn derive_serialize_deserialize() {
    bitflags! {
        #[derive(FlagsSerialize, FlagsDeserialize, PartialEq, Eq, Debug)]
        struct Flags: u8 {
            const A = 1;
            const B = 1 << 1;
            const C = 1 << 2;
        }
    }

    assert_tokens(&Flags::empty().readable(), &[Str("")]);

    assert_tokens(&Flags::empty().compact(), &[U8(0)]);

    assert_tokens(&(Flags::A | Flags::B).readable(), &[Str("A | B")]);

    assert_tokens(&(Flags::A | Flags::B).compact(), &[U8(1 | 2)]);
}
