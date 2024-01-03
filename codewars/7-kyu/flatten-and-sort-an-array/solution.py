# https://www.codewars.com/kata/57ee99a16c8df7b02d00045f

# solution 1: 542ms
def flatten_and_sort(array):
    return sorted([item for subarray in array for item in subarray])