fn biggest(l: &[i32]) -> i32 {
    if l.len() == 2{
        if l[0] > l[1]{
            return l[0];
        }
        return l[1];
    }
    let biggest_of_rest = biggest(&l[1..]);
    if l[0] > biggest_of_rest{
        l[0]
    } else {
        biggest_of_rest
    }
}

fn main() {
    println!("Biggest number is: {}", biggest(&[7, 6, 1, 8, 2, 5, 3, 4]));
}
