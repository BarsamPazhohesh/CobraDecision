package main

import (
	repo "GolangProjs/Repo"
	"fmt"
)

func main() {
	sampleInput := []int{2, 3, 1, 1, 4}
	fmt.Println(repo.CanJump(sampleInput))
}
