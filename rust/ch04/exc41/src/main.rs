fn sum(i: i32) -> i32 {
    if i == 1 {
        1
    } else {
        i + sum(i - 1)
    }
}

fn main() {
    println!("{}", sum(10));
}
