# https://www.codewars.com/kata/5526fc09a1bbd946250002dc

# solution 1: 483ms
def find_outlier(integers):
    even, odd = [], []
    for n in integers:
        if n % 2 == 0:
            even.append(n)
        else:
            odd.append(n)
    if len(even) == 1:
        return even[0]
    else:
        return odd[0]