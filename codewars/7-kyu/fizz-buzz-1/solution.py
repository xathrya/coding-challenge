# https://www.codewars.com/kata/51dda84f91f5b5608b0004cc
# There are multiple kata with Fizz Buzz name

# solution 1: 1080ms
def solution(n):
    a = b = c = 0
    for i in range(1,n):
        if i % 15 == 0:
            c += 1
        elif i % 3 == 0:
            a += 1
        elif i % 5 == 0:
            b += 1
    return [a,b,c]


# solution 2: 471ms
def solution(n):
    a = (n - 1) // 3
    b = (n - 1) // 5
    c = (n - 1) // 15
    return [a - c, b - c, c]