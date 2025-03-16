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

	for key := range *seenValues {
		returnString += fmt.Sprint(key) + ","
	}

	closeReturnString(&returnString, len(*seenValues), inputArrayLen)
func openResultString(returnString *string, len int) {

	*returnString = fmt.Sprint(len)
	*returnString += ", nums = ["

}

func closeReturnString(returnString *string, seenValuesLen, inputArrayLen int) {

	emptyHandler(returnString, seenValuesLen, inputArrayLen)
	*returnString = (*returnString)[:len(*returnString)-1] + "]"

}

func emptyHandler(returnString *string, seenValuesLen, inputArrayLen int) {

	for i := seenValuesLen; i < inputArrayLen; i++ {
		*returnString += "_,"

	}
}
