# https://www.codewars.com/kata/5861d28f124b35723e00005e

# solution 1: 523ms
def zero_fuel(distance_to_pump, mpg, fuel_left):
    return distance_to_pump <= mpg * fuel_left