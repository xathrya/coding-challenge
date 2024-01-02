# https://www.codewars.com/kata/5583090cbe83f4fd8c000051

# solution 1: 512ms
def digitize(n):
    return [ord(c)-48 for c in str(n)][::-1]