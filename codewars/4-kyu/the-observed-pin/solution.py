# https://www.codewars.com/kata/5263c6999e0f40dee200059d

# solution 1: 1167ms
from itertools import product

ADJACENT = ('08', '124', '2135', '326', '4157', '52468', '6359', '748', '85790', '968')

def get_pins(observed):
    return [''.join(p) for p in product(*(ADJACENT[int(d)] for d in observed))]


# solution 2: 2155ms
ADJACENT = {
    '0': ['8'], 
    '1': ['2', '4'], 
    '2': ['1', '3', '5'], 
    '3': ['2', '6'], 
    '4': ['1', '5', '7'], 
    '5': ['2', '4', '6', '8'], 
    '6': ['3', '5', '9'], 
    '7': ['4', '8'], 
    '8': ['5', '7', '9', '0'], 
    '9': ['6', '8']
}

def get_pins(observed): 
    if len(observed) == 1:
        return ADJACENT[observed] + [observed]
    
    return [a + b for a in ADJACENT[observed[0]] + [observed[0]] for b in get_pins(observed[1:])]


# solution 3: 2510ms
ADJACENT = [[0,8],[1,2,4],[1,2,3,5],[2,3,6],[1,4,5,7],[2,4,5,6,8],[3,5,6,9],[4,7,8],[5,7,8,9,0],[9,8,6]]

def get_pins(observed):
    res = []
    dfs([], 0, observed, res)
    return res
    
def dfs(path, ind, observed, res):
    if ind == len(observed):
        res += [''.join(str(e) for e in path)]
        return
    for i in range(len(ADJACENT[int(observed[ind])])):
        dfs(path + [ADJACENT[int(observed[ind])][i]], ind+1, observed, res)