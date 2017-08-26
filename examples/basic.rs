#![allow(dead_code)]

#[macro_use] extern crate enum_count;

use enum_count::EnumCount;

#[derive(EnumCount)] enum Aa { }

#[derive(EnumCount)]
enum Bb { B0 = AA_COUNT as _ }

#[repr(u8)]
#[derive(EnumCount)]
enum Cc { C0 = AA_COUNT as _, C1 = BB_COUNT as _ }

#[repr(usize)]
#[derive(EnumCount)]
enum Dd { D0 = AA_COUNT, D1 = BB_COUNT, D2 = CC_COUNT }

fn main() {
	println!("{} {} {} {}", Aa::count(), BB_COUNT, Cc::count(), DD_COUNT);
}