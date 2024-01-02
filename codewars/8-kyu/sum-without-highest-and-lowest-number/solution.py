# https://www.codewars.com/kata/576b93db1129fcf2200001e6

# solution 1: 686ms
def sum_array(arr):
    if arr is None or len(arr) <= 1:
        return 0
    
    return sum(arr) - min(arr) - max(arr)