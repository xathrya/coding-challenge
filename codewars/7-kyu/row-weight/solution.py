# https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9

# solution 1: 533ms
def row_weights(array):
    weights = [0, 0]
    for i, weight in enumerate(array):
        weights[i % 2] += weight 
    return tuple(weights)


# solution 2: 658ms
def row_weights(array):
    return sum(array[::2]), sum(array[1::2])