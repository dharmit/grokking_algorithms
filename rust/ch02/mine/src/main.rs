// selection_sort takes a slice of integers and returns the index of the smallest item in it
fn selection_sort(nums: &[i32]) -> usize {
    let mut min = nums[0];
    let mut index: usize = 0;

    for i in 0..nums.len() {
        if nums[i] < min {
            min = nums[i];
            index = i as usize;
        }
    }

    return index;
}

fn main() {
    let mut nums: Vec<i32> = vec![];

    for i in 0..100 {
        if i % 2 == 0 {
            nums.push(i * 2);
        } else if i % 3 == 0 {
            nums.push(i*3);
        } else {
            nums.push(i);
        }
    }
    println!("{:?}", nums);

    let mut sorted: Vec<i32> = vec![];
    while nums.len() > 0 {
        let index = selection_sort(&nums);
        sorted.push(nums[index]);

        let mut temp: Vec<i32> = nums[..index].to_vec();
        temp.append(&mut nums[index+1..].to_vec());
        nums = temp;
    }
    println!("{:?}", sorted);
}
