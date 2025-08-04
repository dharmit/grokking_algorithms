package main

func sum(i int) int {
	if i == 1 {
		return 1
	}
	return i + sum(i-1)
}

func main() {
	total := 0

	total = sum(100)
	println(total)
}
