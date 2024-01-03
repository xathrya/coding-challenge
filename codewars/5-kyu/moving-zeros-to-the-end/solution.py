# https://www.codewars.com/kata/52597aa56021e91c93000cb0/

# solution 1: 497ms
def move_zeros(lst):
    return sorted(lst, key=lambda x: x==0 and type(x) is not bool)