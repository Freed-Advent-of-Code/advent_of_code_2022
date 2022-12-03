extern crate core;

mod day_two;

use day_two::rock_paper_scissors;
use day_two::rps_part2;

fn main() {
    let r = rock_paper_scissors();
    println!("{}", r);

    let r2 = rps_part2();
    println!("{}", r2);

}
