# https://www.codewars.com/kata/523f5d21c841566fde000009

# solution 517ms
def array_diff(a, b):
    return list(filter(lambda c: c not in b, a))