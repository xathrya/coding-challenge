# https://www.codewars.com/kata/559d2284b5bb6799e9000047

# solution 1: 665ms
def add_length(str_):
    return [f"{item} {len(item)}" for item in str_.split()]