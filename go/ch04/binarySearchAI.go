/*
	Done entirely by AI after I provided it the `binarySearchRecursive.go` file showing my failed implementation for
	recursive binary search tree.
*/

package main

import "fmt"

// binarySearchRecursive searches for a value in a sorted slice of integers.
// It returns the index of the value, or -1 if not found.
func binarySearchRecursiveAI(nums []int, val int) int {
	// A base case for the recursion: if the slice is empty, the value is not present.
	if len(nums) == 0 {
		return -1
	}

	// Calculate the middle index.
	// Note: We don't need low and high since we are creating new slices on each call.
	mid := len(nums) / 2

	// If the middle element is the value, we've found it.
	if val == nums[mid] {
		return mid
	}

	// If the value is less than the middle element, search in the left half.
	// The slice expression `nums[:mid]` creates a new slice from the beginning up to (but not including) mid.
	if val < nums[mid] {
		return binarySearchRecursiveAI(nums[:mid], val)
	} else {
		// If the value is greater than the middle element, search in the right half.
		// The slice expression `nums[mid+1:]` creates a new slice from `mid+1` to the end.
		result := binarySearchRecursiveAI(nums[mid+1:], val)

		// If the recursive call didn't find the element (result is -1), we propagate that up.
		if result == -1 {
			return -1
		}

		// *** THIS IS THE KEY FIX ***
		// The index returned from the recursive call (`result`) is relative to the smaller, right-hand slice.
		// To get the correct index in the context of the *current* slice, we must add the offset
		// from where the right-hand slice started, which is `mid + 1`.
		return mid + 1 + result
	}
}

func main() {
	nums := []int{0, 1, 4, 5, 7, 8, 9, 11, 12, 13, 16, 17, 19, 20, 23, 24, 25, 27, 28, 29, 31, 32, 35, 36, 37, 40, 41, 43,
		44, 45, 47, 48, 49, 52, 53, 55, 56, 59, 60, 61, 63, 64, 65, 67, 68, 71, 72, 73, 76, 77, 79, 80, 81, 83, 84, 85,
		88, 89, 91, 92, 95, 96, 97, 99, 100, 104, 108, 112, 116, 117, 120, 124, 128, 132, 135, 136, 140, 144, 148, 152,
		153, 156, 160, 164, 168, 171, 172, 176, 180, 184, 188, 189, 192, 196, 207, 225, 243, 261, 279, 297}

	valueToFind := 297
	index := binarySearchRecursiveAI(nums, valueToFind)

	if index != -1 {
		fmt.Printf("Found %d at index %d\n", valueToFind, index)
		fmt.Printf("Value at index %d is %d\n", index, nums[index])
	} else {
		fmt.Printf("Value %d not found in the array\n", valueToFind)
	}
}
