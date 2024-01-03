# https://www.codewars.com/kata/554b4ac871d6813a03000035

# solution 1: 513ms
def high_and_low(numbers):
    numbers = [int(c) for c in numbers.split()]
    return f"{max(numbers)} {min(numbers)}"