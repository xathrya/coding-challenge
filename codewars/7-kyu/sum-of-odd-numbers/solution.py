# https://www.codewars.com/kata/55fd2d567d94ac3bc9000064

# solution 1: 627ms
def row_sum_odd_numbers(n):
    return n**3

# solution 1: 491ms
def row_sum_odd_numbers(n):
    return sum(range(n*(n-1)+1, n*(n+1), 2))