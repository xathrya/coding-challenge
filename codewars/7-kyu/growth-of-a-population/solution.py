# https://www.codewars.com/kata/563b662a59afc2b5120000c6

# solution 1: 531ms
import math
def nb_year(p0, percent, aug, p):
    years = 0
    population = p0 
    while population < p:
        population += math.floor(population * percent/100) + aug
        years += 1
    
    return years