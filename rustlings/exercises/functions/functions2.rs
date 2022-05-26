// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// I AM DONE

fn main() {
    let y = 6;
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
