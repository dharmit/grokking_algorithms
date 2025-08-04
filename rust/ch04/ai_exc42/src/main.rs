fn biggest(l: &[i32]) -> Option<i32> {
    match l.split_first() {
        // no elements
        None => None,
        // single element
        Some((first, rest)) if rest.is_empty() => Some(*first),
        // recursive
        Some((first, rest)) => {
            let biggest_of_rest = biggest(rest);
            Some(std::cmp::max(*first, biggest_of_rest.unwrap()))
        }
    }
}

fn main() {
    let numbers = &[7, 6, 1, 8, 2, 5, 3, 4];

    match biggest(numbers) {
        None => println!("The list is empty"),
        Some(biggest) => println!("The biggest number is {}", biggest),
    }

    let empty: &[i32] = &[];
    match biggest(empty) {
        None => println!("The list is empty"),
        Some(biggest) => println!("The biggest number is {}", biggest),
    }
}
