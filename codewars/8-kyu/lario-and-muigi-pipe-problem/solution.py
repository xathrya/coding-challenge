# https://www.codewars.com/kata/56b29582461215098d00000f

# solution 1: 578ms
def pipe_fix(nums):
    result = []
    for i in range(len(nums) - 1):
        result.append(nums[i])
        pivot = nums[i + 1]
        while nums[i+1] > pivot:
            result.append(pivot)
            pivot += 1
    result.append(nums[-1])
    
    return result 
