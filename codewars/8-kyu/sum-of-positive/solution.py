# https://www.codewars.com/kata/5715eaedb436cf5606000381

# solution 1: 545ms
def positive_sum(arr):
    return sum(filter(lambda num: num > 0, arr))