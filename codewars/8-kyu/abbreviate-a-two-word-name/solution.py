# https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3

# solution 1: 505ms
def abbrev_name(name):
    return '.'.join(w[0] for w in name.split()).upper()