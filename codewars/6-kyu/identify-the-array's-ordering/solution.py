# https://www.codewars.com/kata/57f669477c9a2b1b9700022d

# solution 1: 547ms
def order_type(arr):
    n = []
    if len(arr) == 0:
        return 'Constant'

    for i in arr:
        if type(i) == int:
            i = str(i)
        n.append(len(i))

    if n[0] == n[-1]:
        for i in n:
            if i != n[0]:
                return 'Unsorted'
        return 'Constant'
    elif n[0] < n[-1]:
        for i, ele in enumerate(n[1:]):
            if ele < n[i]:
                return 'Unsorted'
        return 'Increasing'
    elif n[0] > n[-1]:
        for i, ele in enumerate(n[1:]):
            if ele > n[i]:
                return 'Unsorted'
        return 'Decreasing'


# solution 2: 
def order_type(arr):
    if not arr:
        return 'Constant'
    
    arr = list(map(len, [str(el) if type(el)==int else el for el in arr]))
    cmp = sorted(arr)

    if arr == [arr[0]]*len(arr):    s = 'Constant'
    elif arr == cmp:                s = 'Increasing'
    elif arr == cmp[::-1]:          s = 'Decreasing'
    else:                           s = 'Unsorted'

    return s 