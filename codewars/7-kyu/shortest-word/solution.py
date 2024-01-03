# https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9

# solution 1: 673ms
def find_short(s):
    lsts = [len(st) for st in s.split()]
    return min(lsts)