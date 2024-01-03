# https://www.codewars.com/kata/5514e5b77e6b2f38e0000ca9

# solution 1: 674ms
def up_array(arr):
    if len(arr) < 1:
        return None 
    
    for i in arr:
        if len(str(i)) > 1 or i < 0:
            return None 
    
    result = [int(i) for i in str(int("".join([str(n) for n in arr]))+1)]
    return [0] * (len(arr) - len(result)) + result


# solution 2:
def up_array(arr):
    if not arr or min(arr) < 0 or max(arr) > 9:
        return None 
    
    for i in range(len(arr)-1, -1, -1):
        arr[i] += 1
        if arr[i] < 10:
            return arr 
        else:
            arr[i] = 0
    
    return [1]+arr 