package main

import (
	"fmt"
)

func main() {
	sampleInput := []int{2, 3, 1, 1, 4}
	fmt.Println(canJump(sampleInput))
}
func canJump(input []int) bool {
	inputLen := len(input)
	i := 0
	indexValue := input[i]
	for i <= inputLen {
		if i >= inputLen-1 {
			return true
		}
		if cal(&i, &indexValue, input[i]); indexValue == 0 {
			return false
		}
	}
	return false
}
