# https://www.codewars.com/kata/53daa9e5af55c184db00025f

# solution 1: 487ms
def is_prime(num):
    if num < 2:
        return False
    elif num == 2:
        return True
    else:
        for i in range(2, int(num ** 0.5) + 1):
            if num % i == 0:
                return False
        return True