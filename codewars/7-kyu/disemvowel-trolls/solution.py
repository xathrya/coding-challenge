# https://www.codewars.com/kata/52fba66badcd10859f00097e

# solution 1: 494ms
def disemvowel(string_):
    vowels = ['a','e','i','u','o']
    result = [c for c in string_ if c.lower() not in vowels]
    return ''.join(result)