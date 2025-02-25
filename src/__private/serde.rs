extern crate serde;

use bitflags::{
    parser::{self, ParseHex, WriteHex},
    Flags,
};

use core::{fmt, str};

pub use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

struct AsDisplay<'a, B>(pub(crate) &'a B);

impl<'a, B: Flags> fmt::Display for AsDisplay<'a, B>
where
    B::Bits: WriteHex,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        parser::to_writer(self.0, f)
    }
}

/**
Serialize a set of flags as a human-readable string or their underlying bits.

Any unknown bits will be retained.
*/
pub fn serialize<B: Flags, S: Serializer>(flags: &B, serializer: S) -> Result<S::Ok, S::Error>
where
    B::Bits: WriteHex + Serialize,
{
    // Serialize human-readable flags as a string like `"A | B"`
    if serializer.is_human_readable() {
        serializer.collect_str(&AsDisplay(flags))
    }
    // Serialize non-human-readable flags directly as the underlying bits
    else {
        flags.bits().serialize(serializer)
    }
}

/**
Deserialize a set of flags from a human-readable string or their underlying bits.

Any unknown bits will be retained.
*/
pub fn deserialize<'de, B: Flags, D: Deserializer<'de>>(deserializer: D) -> Result<B, D::Error>
where
    B::Bits: ParseHex + Deserialize<'de>,
{
    if deserializer.is_human_readable() {
        // Deserialize human-readable flags by parsing them from strings like `"A | B"`
        struct FlagsVisitor<B>(core::marker::PhantomData<B>);

        impl<'de, B: Flags> Visitor<'de> for FlagsVisitor<B>
        where
            B::Bits: ParseHex,
        {
            type Value = B;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a string value of `|` separated flags")
            }

            fn visit_str<E: Error>(self, flags: &str) -> Result<Self::Value, E> {
                parser::from_str(flags).map_err(|e| E::custom(e))
            }
        }

        deserializer.deserialize_str(FlagsVisitor(Default::default()))
    } else {
        // Deserialize non-human-readable flags directly from the underlying bits
        let bits = B::Bits::deserialize(deserializer)?;

        Ok(B::from_bits_retain(bits))
    }
}
