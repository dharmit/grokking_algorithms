package main

import "fmt"

func binSearch(nums []int, num int) {
	low := 0
	high := len(nums)

	for low <= high {
		mid := (low + high) / 2
		n := nums[mid]
		if n == num {
			fmt.Println(mid)
			return
		} else if n < num {
			low = mid + 1
		} else {
			high = mid - 1
		}
	}
}

func main() {
	s := make([]int, 100)
	for i := range s {
		s[i] = i + 1
	}

	binSearch(s, 42)
}
