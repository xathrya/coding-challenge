# https://www.codewars.com/kata/58ca77b9c0d640ecd2000b1e

# solution 1: 524ms
def procedure(i):
    multiple = list(range(i, 101, i))
    digitize = lambda x: sum(list(map(int, str(x))))
    return sum(map(digitize, multiple))