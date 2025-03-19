''''''''''''''''''''' JUMP GAME '''''''''''''''''''''


nums = []
userNums = input("Enter the numbers separated by commas: ")

for i in userNums.split(","):
    nums.append(int(i))

print(f"nums are: {nums}") # debug

def canJump(nums):
    maxReach = 0
    for i in range(len(nums)):
        if i > maxReach:
            return False
        maxReach = max(maxReach, i + nums[i])
        print(f"maxReach is: {maxReach}") # debug
    return True


print(f"can we jump: {canJump(nums)}") # result
