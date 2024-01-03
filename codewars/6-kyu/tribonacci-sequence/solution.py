# https://www.codewars.com/kata/556deca17c58da83c00002db

# solution 1: 3273ms
def tribonacci(signature, n):
    lut = signature
    for i in range(3, n):
        lut.append(lut[i-1] + lut[i-2] + lut[i-3])
    return lut[:n]