# https://www.codewars.com/kata/578aa45ee9fd15ff4600090d

# solution 1: 505ms
def sort_array(source_array):
    odd = sorted(filter(lambda n: n % 2 == 1, source_array))
    
    j = 0
    for i in range(len(source_array)):
        if source_array[i] % 2 == 1:
            source_array[i] = odd[j]
            j += 1
    
    return source_array
    