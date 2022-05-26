// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// I AM  DONE

fn main() {
    let mut answer = square(3);
    answer = square(3) - 1;
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
