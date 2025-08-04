fn binary_search_recursive(nums: &[i32], value: i32) -> i32 {
    if nums.len() == 0 {
        return -1;
    }
    
    let mid = nums.len() / 2;

    if value == nums[mid] {
        return mid as i32;
    }

    if value < nums[mid] {
        return binary_search_recursive(&nums[0..mid], value);
    } else {
        let result = binary_search_recursive(&nums[mid+1..], value);

        if result == -1 {
            return -1;
        }

        return mid as i32 + 1 + result;
    }
}

fn main() {
    let nums = [0, 1, 4, 5, 7, 8, 9, 11, 12, 13, 16, 17, 19, 20, 23, 24, 25, 27, 28, 29, 31, 32, 35, 36, 37, 40, 41, 43, 
		44, 45, 47, 48, 49, 52, 53, 55, 56, 59, 60, 61, 63, 64, 65, 67, 68, 71, 72, 73, 76, 77, 79, 80, 81, 83, 84, 85, 
		88, 89, 91, 92, 95, 96, 97, 99, 100, 104, 108, 112, 116, 117, 120, 124, 128, 132, 135, 136, 140, 144, 148, 152, 
		153, 156, 160, 164, 168, 171, 172, 176, 180, 184, 188, 189, 192, 196, 207, 225, 243, 261, 279, 297];
    
    let value_to_find = 24;
    let index = binary_search_recursive(&nums, value_to_find);

    if index == -1 {
        println!("{} not found", value_to_find)
    } else {
        println!("{} found at index {}", value_to_find, index);
        println!("value at index {} is {}", index, nums[index as usize])
    }
}
