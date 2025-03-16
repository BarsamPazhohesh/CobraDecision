package repo

import "fmt"

func Remove_Duplicates_from_Sorted_Array(inputArray []int) {

	seenValues := make(map[int]int)

	for i := 0; i < len(inputArray); i++ {
		seenValues[inputArray[i]] += 1
	}
}

