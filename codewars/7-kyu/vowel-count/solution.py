# https://www.codewars.com/kata/54ff3102c1bad923760001f3

# solution 1: 518ms
def get_count(sentence):
    return sum(1 if c in "aeiou" else 0 for c in sentence)