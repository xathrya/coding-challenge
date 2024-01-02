# https://www.codewars.com/kata/58649884a1659ed6cb000072

# solution 1: 528ms
def update_light(current):
    return {"green":"yellow", "yellow":"red", "red":"green"}[current]