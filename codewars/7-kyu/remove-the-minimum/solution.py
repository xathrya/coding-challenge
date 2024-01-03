# https://www.codewars.com/kata/563cf89eb4747c5fb100001b

# solution 1: 657ms
def remove_smallest(numbers):
    result = numbers.copy()
    if len(result) > 0:
        result.remove(min(result))
    return result