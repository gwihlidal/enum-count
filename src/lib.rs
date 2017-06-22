#[allow(unused_imports)]
#[macro_use] extern crate enum_count_derive;
pub use enum_count_derive::*;

pub trait EnumCount {
	fn count() -> usize;
}