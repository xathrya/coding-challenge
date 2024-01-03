# https://www.codewars.com/kata/5865a407b359c45982000036

# solution 1: 997ms
from itertools import permutations

def slogan_maker(array):
    # remove the duplicate
    array = list(dict.fromkeys(array))
    
    # do permutation
    return list(map(" ".join, permutations(array, len(array))))