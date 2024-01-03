# https://www.codewars.com/kata/586beb5ba44cfc44ed0006c3

# solution 1: 630ms
def sum_even_numbers(seq):
    return sum(c for c in seq if c%2==0)