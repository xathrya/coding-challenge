# https://www.codewars.com/kata/5254ca2719453dcc0b00027d

# solution 1: 1046ms 
import itertools
def permutations(s):
    return list(set("".join(it) for it in itertools.permutations(s)))