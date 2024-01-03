# https://www.codewars.com/kata/563f037412e5ada593000114

# solution 1: 551ms
def calculate_years(principal, interest, tax, desired):
    years = 0
    money = principal
    while money < desired:
        money += (money * interest) * (1-tax)
        years += 1
    
    return years 