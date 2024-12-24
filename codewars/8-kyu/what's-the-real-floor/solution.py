# 

# solution 1: 476ms
def get_real_floor(n):
    if n <= 0:
        return n
    
    return n - 1 if n > 0 and n < 13 else n - 2

# solution 2: 511ms
def get_real_floor(n):
    return n - 1 if n > 0 and n < 13 else n - 2 if n > 13 else n
