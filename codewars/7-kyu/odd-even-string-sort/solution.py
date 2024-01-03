# https://www.codewars.com/kata/580755730b5a77650500010c

# solution 1: 557ms
def sort_my_string(s):
    return s[0::2] + " " + s[1::2]