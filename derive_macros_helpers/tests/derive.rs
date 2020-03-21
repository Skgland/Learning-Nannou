use derive_macros::{Bounded, Enumerable};
use derive_macros_helpers::{Bounded, Enumerable};

#[derive(Bounded, Enumerable)]
pub enum Test {
    Variant(i8),
}
