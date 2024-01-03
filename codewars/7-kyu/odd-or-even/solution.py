# https://www.codewars.com/kata/5949481f86420f59480000e7

# solution 1: 526ms
def odd_or_even(arr):
    return "even" if sum(arr) % 2 == 0 else "odd" 