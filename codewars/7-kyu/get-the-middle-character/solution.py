# https://www.codewars.com/kata/56747fd5cb988479af000028

# solution 1: 523ms
def get_middle(s):
    n = len(s)
    m = n//2
    if n % 2 == 0:
        return s[m-1:m+1]
    else:
        return s[m]