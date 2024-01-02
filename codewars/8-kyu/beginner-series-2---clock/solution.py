# https://www.codewars.com/kata/55f9bca8ecaa9eac7100004a

# solution 1: 525ms
def past(h, m, s):
    return (((h * 60) + m) * 60 + s) * 1000