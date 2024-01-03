# https://www.codewars.com/kata/58e0f0bf92d04ccf0a000010

# solution 1: 527ms
def lost_sheep(friday,saturday,total):
    return total - sum(friday) - sum(saturday)