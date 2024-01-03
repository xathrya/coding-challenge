# https://www.codewars.com/kata/59f7597716049833200001eb

# solution 1: 5899ms
def solve(a, b):
    nums = {"0": "0", "1":"1", "6":"9", "8":"8", "9":"6"}
    total = 0
    for i in range(a, b):
        i = str(i)
        m = "".join(nums[x] for x in i[::-1] if x in nums)
        if i == m:
            total += 1
    return total 

# solution 2: 
def solve(a, b):
    return sum(str(n) == str(n)[::-1].translate(str.maketrans('2345679', 'XXXX9X6')) for n in range(a, b))