# https://www.codewars.com/kata/592915cc1fad49252f000006

# solution 1: 461ms
def no_ifs_no_buts(a, b):
    return {
        a == b: str(a) + " is equal to " + str(b),
        a < b: str(a) + " is smaller than " + str(b),
        a > b: str(a) + " is greater than " + str(b),
    }[True]

# solution 2: 498ms
# using list and indexing
def no_ifs_no_buts(a, b):
    return ([
        '%d is smaller than %d',
        '%d is equal to %d',
        '%d is greater than %d'
    ][int(a >= b) + int(a > b)] % (a, b))

# solution 3: 506ms
def no_ifs_no_buts(a, b):
    compare = {
        -1: " is smaller than ",
        0:  " is equal to ",
        1:  " is greater than "
    }
    return str(a) + compare[(a > b) - (a < b)] + str(b)