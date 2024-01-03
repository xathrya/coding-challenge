# https://www.codewars.com/kata/558fc85d8fd1938afb000014

# solution 1: 697ms
def sum_two_smallest_numbers(numbers):
    return sum(sorted(numbers)[:2])