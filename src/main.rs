mod core;

use core::{Board, Direction};

const N: usize = 6;

fn p_array(table: &[[i8; N]; N]) {
    for row in table {
        println!("{:?}", row);
    }
}

fn main() {
    let mut a = Board::<N>::new(3).unwrap();
    p_array(a.get_table());

    a.rotation(Direction::Down);
    while a.walk() {
        p_array(a.get_table());
    }

    println!("game over");
    p_array(a.get_table());

    println!("{}", a.get_score());
}
