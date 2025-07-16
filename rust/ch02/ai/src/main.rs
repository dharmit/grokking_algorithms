fn selection_sort<T: Ord>(arr: &mut[T]){
    let len = arr.len();

    // iterate upto second last item only because by the time we reach the last eleemnt the slice will already be sorted
    for i in 0..len{
        // starting from i (i changes with for) up to end of the slice, find the index of the minimum element
        // iter() helps iterate over the slice
        // enumerate() gives us (index, value) pairs
        // min_by_key() finds pair with minimum value
        // map() extracts just the index from the resulting pair with minimum in value
        // unwrap() is safe here as slice won't be empty within the loop's bounds
        let min_index_in_rest = arr[i..]
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| v)
            .map(|(idx, _)| idx)
            .unwrap_or(0);
        
        // min_index_in_rest value will be relative to that of i's; so add i to find actual index
        let min_index = i + min_index_in_rest;

        // if the two index are not same, swap the values
        if i != min_index{
            arr.swap(i, min_index);
        }
    }
}

fn main() {
    let mut nums: Vec<i32> = (0..100)
        .map(|i| {
            if i%2==0{
                i*2
            } else if i%3==0 {
                i*3
            } else{
                i
            }
        }).collect();
    println!("{:?}", nums);
    selection_sort(&mut nums);
    println!("{:?}", nums);

    let mut words = vec!["rust", "is", "a", "systems", "programming", "language"];
    println!("{:?}", words);
    selection_sort(&mut words);
    println!("{:?}", words);
}
