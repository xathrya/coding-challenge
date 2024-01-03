# https://www.codewars.com/kata/550554fd08b86f84fe000a58

# solution 1: 778ms
def in_array(array1, array2):
    result = []
    for arr2 in array2:
        for arr1 in array1:
            if arr1 in arr2 and arr1 not in result:
                result.append(arr1)
    
    return sorted(result)


# solution 2: 570ms
def in_array(array1, array2):
    return sorted({sub for sub in array1 if any(sub in arr2 for arr2 in array2)})