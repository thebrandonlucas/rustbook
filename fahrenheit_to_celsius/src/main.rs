fn f_to_c(f: i32) -> i32 {
    (f - 32) / (9 / 5)
}

fn c_to_f(c: i32) -> i32 {
    (c * (9 / 5)) + 32
}

fn main() {
    let f = 81;

    let converted_celsius = f_to_c(f);
    println!("{f} degrees fahrenheit is {converted_celsius}");

    let converted_fahrenheit = c_to_f(converted_celsius);
    println!("{converted_celsius} degrees celsius is {converted_fahrenheit}");
}
