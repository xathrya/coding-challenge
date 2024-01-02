# https://www.codewars.com/kata/5b077ebdaf15be5c7f000077

# solution 1: 547ms
def count_sheep(n):
    return "".join(f"{i} sheep..." for i in range(1, n+1))