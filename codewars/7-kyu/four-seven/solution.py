# https://www.codewars.com/kata/5ff50f64c0afc50008861bf0

# solution 1: 517ms
def solution(n):
    return {4:7, 7:4}.get(n, False)