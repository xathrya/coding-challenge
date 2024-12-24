# https://www.codewars.com/kata/513e08acc600c94f01000001

# solution 1: 501ms
def rgb(r, g, b):
    correct = lambda x: min(255, max(x, 0))
    return f"{correct(r):02X}{correct(g):02X}{correct(b):02X}"

