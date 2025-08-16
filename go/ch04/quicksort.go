package main

import "fmt"

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

	fmt.Println("sorted:", quicksort(nums))
}

func quicksort(nums []int) []int {
	length := len(nums)
	if length < 2 {
		return nums
	}

	copynums := make([]int, len(nums))
	copy(copynums, nums)

	mid := length / 2

	var lesser []int
	var bigger []int

	for i := 0; i < mid; i++ {
		if copynums[i] > copynums[mid] {
			bigger = append(bigger, copynums[i])
		} else {
			lesser = append(lesser, copynums[i])
		}
	}
	for i := mid + 1; i < length; i++ {
		if copynums[i] > copynums[mid] {
			bigger = append(bigger, copynums[i])
		} else {
			lesser = append(lesser, copynums[i])
		}
	}

	lesser = quicksort(lesser)
	bigger = quicksort(bigger)
	ret := append([]int{}, lesser...)
	ret = append(ret, copynums[mid])
	return append(ret, bigger...)
}
