# https://www.codewars.com/kata/59a96d71dbe3b06c0200009c

# solution 1: 509ms
def generate_shape(n):
    return "\n".join(["+" * n] * n)