# https://www.codewars.com/kata/556025c8710009fc2d000011

# solution 1: 533ms
mapper = {
    "a":"α", "b":"β", "d":"δ",
    "e":"ε", "i":"ι", "k":"κ",
    "n":"η", "o":"θ", "p":"ρ",
    "r":"π", "t":"τ", "u":"μ",      
    "v":"υ", "w":"ω", "x":"χ",
    "y":"γ"
}

def gr33k_l33t(string):
    return "".join(mapper.get(c, c) for c in string.lower())


# solution 2: 626ms
mapper = str.maketrans("abdeiknoprtuvwxy", "αβδεικηθρπτμυωχγ")

def gr33k_l33t(string):
    return string.lower().translate(mapper)