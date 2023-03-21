// Calculate the nth fibonacci number
fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut prev = 0;
        let mut fibnum: i32 = 1;
        for _ in 1..n {
            let tmp = fibnum;
            fibnum += prev;
            prev = tmp;
        }
        fibnum
    }
}

fn main() {
    let n = 10; // get the 10th fib number

    let result = fib(n);

    println!("The {n}th fibonacci number is {result}");
}
