# https://www.codewars.com/kata/54da5a58ea159efa38000836

# solution 1: 662ms
# create dictionary of item and occurrence, filter the odds, get the first element
def find_it(seq):
    D = {x: seq.count(x) for x in seq}
    return list(filter(lambda e: e[1] % 2 == 1, D.items()))[0][0]

# solution 2: 505ms
# iterate seq and find the first item which has odd occurrence
def find_it(seq):
    for e in seq:
        if seq.count(e) % 2 == 1:
            return e 

# solution 3: 495ms
def find_it(seq):
    return [x for x in seq if seq.count(x) % 2][0]