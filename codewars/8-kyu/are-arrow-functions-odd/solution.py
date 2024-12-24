# https://www.codewars.com/kata/are-arrow-functions-odd

# solution 1: 534ms
odds = lambda l: [x for x in l if x % 2 != 0]

# solution 2: 483ms
odds = lambda l: list(filter(lambda x: x % 2 != 0, l))
