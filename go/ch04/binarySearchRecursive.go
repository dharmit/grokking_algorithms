/*
	Failed attempt to implement recusrive binary search. Look at `binarySearchAI.go` for correct implementation
*/

package main

func binarySearchRecursive(nums []int, val int) int {
	var index int
	low := 0
	high := len(nums)
	mid := (low + high) / 2

	if val == nums[mid] {
		return mid
	}
	if val < nums[mid] {
		index = len(nums) - mid - binarySearchRecursive(nums[:mid-1], val)
	} else {
		index = mid + binarySearchRecursive(nums[mid+1:], val)
	}
	return index
}

func main() {
	nums := []int{0, 1, 4, 5, 7, 8, 9, 11, 12, 13, 16, 17, 19, 20, 23, 24, 25, 27, 28, 29, 31, 32, 35, 36, 37, 40, 41, 43,
		44, 45, 47, 48, 49, 52, 53, 55, 56, 59, 60, 61, 63, 64, 65, 67, 68, 71, 72, 73, 76, 77, 79, 80, 81, 83, 84, 85,
		88, 89, 91, 92, 95, 96, 97, 99, 100, 104, 108, 112, 116, 117, 120, 124, 128, 132, 135, 136, 140, 144, 148, 152,
		153, 156, 160, 164, 168, 171, 172, 176, 180, 184, 188, 189, 192, 196, 207, 225, 243, 261, 279, 297}

	index := binarySearchRecursive(nums, 9)
	println(index, nums[index])
}
