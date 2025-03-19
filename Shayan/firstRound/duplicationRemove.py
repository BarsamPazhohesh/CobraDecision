'''''''''''''Remove Duplicates from Sorted Array'''''''''''''''
# criating a func to remove duplicates in users givin array
def removeDuplicates(nums): 
    if not nums:
        return 0
    
    # the index for the next unique element
    unique_index = 1
    
    # Iterate through the array starting from the second element
    for i in range(1, len(nums)):
        # If the current element is different from the last unique element
        if nums[i] != nums[unique_index - 1]:
            # Update the array at the unique index with the current element
            nums[unique_index] = nums[i]
            # Move to the next unique index
            unique_index += 1
            
    # Return the length of the array with unique elements
    return unique_index
nums = []
# user array input
userNums = input("Enter the numbers separated by commas: ")
# Convert the input string to a list of integers
for i in userNums.split(","):
    nums.append(int(i))
print(f"'debug' nums are: {nums}") # debug

# Call the function and get the new length
new_length = removeDuplicates(nums)
# Print the result
print(f"'debug' Array after removing duplicates: {nums[:new_length]}") # debug
# Print the new length of the array
print(f"New length of the array: {new_length}") # result

