# https://www.codewars.com/kata/5648b12ce68d9daa6b000099

# solution 1: 483ms
def number(bus_stops):
    total = list(map(sum, zip(*bus_stops)))
    return total[0] - total[1]