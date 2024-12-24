# https://www.codewars.com/kata/5d5ee4c35162d9001af7d699

# solution 1: 630ms
def sum_of_minimums(numbers):
    return sum(min(row) for row in numbers)


# solution 2: 570ms
def sum_of_minimums(numbers):
    return sum(map(min, numbers))


# solution 3: 
# this is very straight forward
def sum_of_minimums(numbers):
    total = 0
    for nums in numbers:
        total += min(nums)
    return total