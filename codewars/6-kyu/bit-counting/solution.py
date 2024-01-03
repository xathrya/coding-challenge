# https://www.codewars.com/kata/526571aae218b8ee490006f4

# solution 1: 513ms
def count_bits(n):
    return bin(n).count("1")