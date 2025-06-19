#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod kirby;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    kirby::install();
}