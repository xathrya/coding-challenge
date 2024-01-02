# https://www.codewars.com/kata/57eae65a4321032ce000002d

# solution 1: 519ms
def fake_bin(x):
    return "".join("0" if int(c) < 5 else "1" for c in x)