# https://www.codewars.com/kata/5700c9acc1555755be00027e/

# solution 1: 564ms
def contain_all_rots(text, arr):
    return all(text[i:] + text[:i] in arr for i in range(len(text)))