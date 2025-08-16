fn main() {
    let mut nums: Vec<i32> = Vec::new();
    for i in 0..100i32 {
        let new_val = match(i % 3, i%2) {
            (0, _) => i * 3,
            (_, 0) => i * 2,
            (_, _) => i,
        };
        nums.push(new_val);
    }
    println!("original: {:?}\n", nums);
    println!("sorted: {:?}", quicksort(&nums[..]));
}

fn quicksort(nums: &[i32]) -> Vec<i32> {
    let length = nums.len();
    if length < 2 {
        return nums.to_vec();
    }
    let mid = length/2;
    let mut lesser: Vec<i32> = Vec::new();
    let mut bigger: Vec<i32> = Vec::new();

    for i in 0..mid {
        if nums[i] > nums[mid] {
            bigger.push(nums[i]);
        } else {
            lesser.push(nums[i]);
        }
    }
    for i in mid+1..nums.len() {
        if nums[i] > nums[mid] {
            bigger.push(nums[i]);
        } else {
            lesser.push(nums[i]);
        }
    }

    bigger = quicksort(&bigger);
    lesser = quicksort(&lesser);
    let mut ret: Vec<i32> = Vec::new();
    ret.append(&mut lesser);
    ret.push(nums[mid]);
    ret.append(&mut bigger);

    return ret;
}