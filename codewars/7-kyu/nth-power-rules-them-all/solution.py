# https://www.codewars.com/kata/58aed2cafab8faca1d000e20

# solution 1: 523ms
def modified_sum(a, n):
    return sum(x**n for x in a) - sum(a)


# solution 2: 475ms
def modified_sum(a, n):
    return sum(map(lambda x: x**n, a)) - sum(a)


# solution 3: 617ms
def modified_sum(a, n):
    return sum(x**n - x for x in a)