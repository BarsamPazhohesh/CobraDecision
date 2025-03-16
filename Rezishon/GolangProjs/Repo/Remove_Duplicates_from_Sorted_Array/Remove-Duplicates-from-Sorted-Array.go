package repo

import "fmt"

// Remove_Duplicates_from_Sorted_Array takes a sorted array of integers as input
// and returns a string with all duplicate elements removed, preserving the
// original order of the elements.
//
// *Parameters:
//
//	inputArray ([]int): The sorted array of integers to be processed.
//
// *Returns:
//
//	No return
func Remove_Duplicates_from_Sorted_Array(inputArray []int) {

	seenValues := make(map[int]int)

	for i := 0; i < len(inputArray); i++ {
		seenValues[inputArray[i]] += 1
	}

	StringBuilder(&seenValues, len(inputArray))
}

// Build the return string as the question need
//
// * Parameters:
//
//	seenValues (*map[int]int): The dictionary like structure
//	inputArrayLen (int): The Len of inputArray
//
// * Returns:
//
//	No return
func StringBuilder(seenValues *map[int]int, inputArrayLen int) {

	var returnString string

	openResultString(&returnString, len(*seenValues))

	for key := range *seenValues {
		returnString += fmt.Sprint(key) + ","
	}

	closeReturnString(&returnString, len(*seenValues), inputArrayLen)

	fmt.Println(returnString)
}

// Add the start of return string
//
// * Parameters:
//
//	returnString (*string): Pointer that points to returnString in StringBuilder
//	seenValuesLen (int): The Len of seenValues
//
// * Returns:
//
//	No return
func openResultString(returnString *string, seenValuesLen int) {

	*returnString = fmt.Sprint(seenValuesLen)
	*returnString += ", nums = ["

}

// Add the end of return string
//
// * Parameters:
//
//	returnString (*string): Pointer that points to returnString in StringBuilder
//	seenValuesLen (int): The Len of seenValues
//	inputArrayLen (int): The Len of inputArray
//
// * Returns:
//
//	No return
func closeReturnString(returnString *string, seenValuesLen, inputArrayLen int) {

	emptyHandler(returnString, seenValuesLen, inputArrayLen)
	*returnString = (*returnString)[:len(*returnString)-1] + "]"

}

// Add the under score char to return string
//
// * Parameters:
//
//	returnString (*string): Pointer that points to returnString in StringBuilder
//	seenValuesLen (int): The Len of seenValues
//	inputArrayLen (int): The Len of inputArray
//
// * Returns:
//
//	No return
func emptyHandler(returnString *string, seenValuesLen, inputArrayLen int) {

	for i := seenValuesLen; i < inputArrayLen; i++ {
		*returnString += "_,"

	}
}
