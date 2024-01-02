# https://www.codewars.com/kata/5a34b80155519e1a00000009

# solution 1: 509ms
def multiple_of_index(arr):
    return [j for i,j in enumerate(arr) if (j==0 and i==0) or (i!=0 and j%i==0)]