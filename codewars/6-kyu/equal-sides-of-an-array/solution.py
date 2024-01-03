# https://www.codewars.com/kata/5679aa472b8f57fb8c000047

# solution 1: 537ms
def find_even_index(arr):
    sl = 0
    sr = sum(arr)
    for idx,itm in enumerate(arr)):
        sr -= itm
        if sl == sr:
            return idx
        sl += itm