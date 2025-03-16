package repo
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
func Cal(i, indexValue *int, value int) {
	*i += *indexValue
	*indexValue = value
}
