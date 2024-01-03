# https://www.codewars.com/kata/5390bac347d09b7da40006f6

# solution 1: 577ms
def to_jaden_case(string):
    return " ".join(w.capitalize() for w in string.split())