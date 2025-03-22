''''''''''''''''' Two Sum '''''''''
# Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
nums = []
target = int(input("Enter the target number: "))
userNums = input("Enter the numbers separated by commas: ")
# userNums = "2,7,11,15"
for i in userNums.split(","):
    nums.append(int(i))

print(f"'debug' nums are: {nums}") # debug

# function to return indices of the two numbers such that they add up to target
def twoSum(nums, target):
    dict = {} # dictionary to store the difference between target and nums[i] and the index i
    for i in range(len(nums)):  # iterate through the list
        if nums[i] in dict: # if the number is in the dictionary, return the index of the number and the index of the number in the dictionary
            return [dict[nums[i]], i] # return the index of the number in the dictionary and the index of the number
        dict[target - nums[i]] = i # store the difference between target and nums[i] and the index i in the dictionary
        print(f"'debug' dict is: {dict}") # debug

print(f"result: {twoSum(nums, target)}") # result
