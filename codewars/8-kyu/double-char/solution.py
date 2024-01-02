# https://www.codewars.com/kata/56b1f01c247c01db92000076

# solution 1: 564ms
def double_char(s):
    return "".join(c * 2 for c in s)