# https://www.codewars.com/kata/57a2013acf1fa5bfc4000921

# solution 1: 696ms
def find_average(numbers):
    return sum(numbers) / len(numbers) if numbers else 0