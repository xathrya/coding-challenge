# https://www.codewars.com/kata/580a41b6d6df740d6100030c

# solution 1: 520ms
def alan(arr):
    subset = { "Rejection", "Disappointment", "Backstabbing Central", "Shattered Dreams Parkway" }
    return "Smell my cheese you mother!" if subset.issubset(arr) else "No, seriously, run. You will miss it."
