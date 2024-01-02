# https://www.codewars.com/kata/57e76bc428d6fbc2d500036d

# solution 1: 536ms
def string_to_array(s):
    return s.split() if s else [""]