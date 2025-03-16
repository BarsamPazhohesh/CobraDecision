package repo

// The main function of CanJump Question
// Handle main logic.
//
// * Parameters:
//
//	input ([]int): The array of int that we goes over it.
//
// * Returns:
//
//	bool: Returns true if we read the end of the input array.
func CanJump(input []int) bool {

	inputLen := len(input)
	i := 0
	indexValue := input[i]

	for i <= inputLen {
		if i >= inputLen-1 {
			return true
		}
		if Cal(&i, &indexValue, input[i]); indexValue == 0 {
			return false
		}
	}
	return false
}

// Calculate which step with what value should we work with in each iteration
//
// * Parameters:
//
//	i (*int): The iteration variable
//	indexValue (*int): Value of each index of the array
//	value (int): The value which we will set to "indexValue"
//
// * Returns:
//
//	No return because it only sets values
func Cal(i, indexValue *int, value int) {
	*i += *indexValue
	*indexValue = value
}
