#[macro_use]
extern crate text_io;

#[macro_use]
extern crate derive_new;

mod deck;
mod card;
mod hand;

fn main() {
    let i: i32 = read!();

    println!(
        "ejemplo de input y output
    , el numero ingresado fue {}",
        i
    );
}
