# https://www.codewars.com/kata/585d7b4685151614190001fd

# solution 1: 474ms
def get_total(costs, items, tax):
    purchase = 0
    for item in items:
        if item in costs:
            purchase += costs[item]
    return round(purchase + purchase * tax, 2)