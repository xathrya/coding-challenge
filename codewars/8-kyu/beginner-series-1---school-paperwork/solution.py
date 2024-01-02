# https://www.codewars.com/kata/55f9b48403f6b87a7c0000bd

# solution 1: 470ms
def paperwork(n, m):
    if n < 0 or m < 0:
        return 0
    else:
        return n * m


# solution 2: 499ms
def paperwork(n, m):
    return 0 if n < 0 or m < 0 else n * m