# https://www.codewars.com/kata/5667e8f4e3f572a8f2000039

# solution 1: 502ms
def accum(s):
    return "-".join([c.upper() + c.lower() * i for i,c in enumerate(s)])