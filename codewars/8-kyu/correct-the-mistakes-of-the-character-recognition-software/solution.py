# https://www.codewars.com/kata/577bd026df78c19bca0002c0

# solution 1: 689ms
def correct(s):
    return s.replace("5", "S").replace("0", "O").replace("1", "I")


# solution 2: 
def correct(s):
    return string.translates(str.maketrans("501", "SOI"))