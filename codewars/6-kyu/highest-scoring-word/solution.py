# https://www.codewars.com/kata/57eb8fcdf670e99d9b000272

# solution 1: 551ms
def high(x):
    return max(x.split(), key=lambda k: sum(ord(c) - 96 for c in k))