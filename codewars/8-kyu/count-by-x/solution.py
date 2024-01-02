# https://www.codewars.com/kata/5513795bd3fafb56c200049e

# solution 1: 595ms
def count_by(x, n):
    return list(range(x, x * n + 1, x))