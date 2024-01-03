# https://www.codewars.com/kata/5467e4d82edf8bbf40000155

# solution 1: 521ms
def descending_order(num):
    return int("".join(sorted(str(num), reverse=True)))