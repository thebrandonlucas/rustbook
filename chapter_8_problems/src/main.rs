use measures_of_center::{mean_declarative, median, mode_declarative};

fn main() {
    let numbers = vec![3, 6, 4, 1, 1, 2, 2, 3, 6, 3, 5, 5, 6];
    let dec_mean = mean_declarative(&numbers);
    let med = median(&numbers);
    let dec_mode = mode_declarative(&numbers);

    println!("The mean is: {dec_mean}");
    println!("The median is: {med}");
    println!("The mode is: {dec_mode}");
}
