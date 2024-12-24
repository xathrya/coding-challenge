# https://www.codewars.com/kata/524f5125ad9c12894e00003f

# solution 1: 521ms
def remainder(a, b):
    if min(a, b) == 0:
        return None 
    elif a > b:
        return a % b
    else:
        return b % a 


# solution 1: 474ms
def remainder(a, b):
    return max(a, b) % min(a, b) if min(a, b) else None