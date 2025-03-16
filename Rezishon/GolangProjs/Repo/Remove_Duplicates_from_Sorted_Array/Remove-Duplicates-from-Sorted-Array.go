package repo

import "fmt"

func Remove_Duplicates_from_Sorted_Array(inputArray []int) {

	seenValues := make(map[int]int)

	for i := 0; i < len(inputArray); i++ {
		seenValues[inputArray[i]] += 1
	}

	StringBuilder(&seenValues, len(inputArray))
}

func StringBuilder(seenValues *map[int]int, inputArrayLen int) {

	var returnString string

	openResultString(&returnString, len(*seenValues))
func openResultString(returnString *string, len int) {
}
}
