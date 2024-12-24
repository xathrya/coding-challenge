# https://www.codewars.com/kata/5ce399e0047a45001c853c2b

# solution 1: 2752ms
def parts_sums(ls):
    result = [sum(ls)]
    for item in ls:
        result.append(result[-1] - item)
    
    return result


# solution 2: 3661ms
from itertools import accumulate
def parts_sums(ls):
    return list(accumulate(ls, lambda x, y: x - y, initial=sum(ls)))


# solution 3: 2520ms
from itertools import accumulate
def parts_sums(ls):
    return [0, *accumulate(reversed(ls))][::-1]