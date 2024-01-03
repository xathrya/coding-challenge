# https://www.codewars.com/kata/55908aad6620c066bc00002a

# solution 1: 523ms
def xo(s):
    s = s.upper()
    return s.count("O") == s.count("X")