mod answer;
mod days;
mod table;

use crate::{
    answer::{total_duration, Answer},
    table::print_table,
};

fn main() {
    let answers: Vec<Answer> = vec![days::day_1::execute(), days::day_2::execute()];

    let (duration, no_io_duration) = total_duration(&answers);

    print_table(answers);

    println!("\nTotal duration: {duration}μs (Total no IO duration {no_io_duration}μs)",);
}
