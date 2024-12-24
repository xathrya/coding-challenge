# https://www.codewars.com/kata/53da6d8d112bd1a0dc00008b

# solution 1: 517ms
def reverse_list(l):
    return l[::-1]

# solution 2: 594ms
def reverse_list(l):
    return list(reversed(l))
