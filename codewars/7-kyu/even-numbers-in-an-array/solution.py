# https://www.codewars.com/kata/5a431c0de1ce0ec33a00000c

# solution 1: 503ms
def even_numbers(arr, n):
    result, arr = [], arr[::-1]
    for i in range(len(arr)):
        if arr[i] % 2 == 0 and len(result) < n:
            result.append(arr[i])
    
    return result[::-1]