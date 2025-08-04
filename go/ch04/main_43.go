package main

func biggest(l []int) int {
	if len(l) == 2 {
		if l[0] > l[1] {
			return l[0]
		}
		return l[1]
	}
	biggestOfRest := biggest(l[1:])
	if l[0] > biggestOfRest {
		return l[0]
	}
	return biggestOfRest
}

func main() {
	l := []int{7, 6, 1, 8, 2, 5, 3, 4}
	println(biggest(l))
}
