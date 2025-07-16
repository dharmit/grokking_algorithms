package main

import "fmt"

// selectionSort finds the minimum number from the given slice and returns its value and index
func selectionSort(nums []int) (value, index int) {
	value = nums[0]
	index = 0

	for i := 0; i < len(nums); i++ {
		if nums[i] < value {
			value = nums[i]
			index = i
		}
	}
	return value, index
}

func main() {
	nums := make([]int, 100)
	for i := range 100 {
		if i%2 == 0 {
			nums[i] = i * 2
		} else if i%3 == 0 {
			nums[i] = i * 3
		} else {
			nums[i] = i
		}
	}
	fmt.Println("original:", nums)

	var sorted []int
	for len(nums) > 0 {
		value, index := selectionSort(nums)
		sorted = append(sorted, value)
		var temp []int
		temp = nums[0:index]
		temp = append(temp, nums[index+1:]...)
		nums = temp
	}
	fmt.Println("sorted:", sorted)
}
