# https://www.codewars.com/kata/514b92a657cdc65150000006

# solution 1: 508ms
def solution(number):
    return sum(c if c % 3 == 0 or c % 5 == 0 else 0 for c in range(1, number))