# https://www.codewars.com/kata/541629460b198da04e000bb9

# solution 1: 457ms
def last(*args):
    l = args[-1]
    if type(l)==list or type(l)==str:
        return l[-1]
    else:
        return l

# solution 2:
from collections import Iterable

def last(*args):
    return args[-1][-1] if isinstance(args[-1], Iterable) else args[-1]