# https://www.codewars.com/kata/53dbd5315a3c69eed20002dd

# solution 1: 471ms
def filter_list(l):
    return [a for a in l if isinstance(a, int)]