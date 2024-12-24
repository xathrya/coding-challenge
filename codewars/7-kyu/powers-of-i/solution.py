# https://www.codewars.com/kata/5a97387e5ee396e70a00016d
# other solution is variant using tuple and dictionary

# solution 1: 457ms
def pofi(n):
    return ['1', 'i', '-1', '-i'][n % 4]