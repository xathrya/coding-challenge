# https://www.codewars.com/kata/576bb71bbbcf0951d5000044

# solution 1: 495ms
def count_positives_sum_negatives(arr):
    if not arr: return []
    
    lp = len(list(filter(lambda it: it > 0, arr)))
    sn = sum(filter(lambda it: it < 0, arr))
    return [lp, sn]