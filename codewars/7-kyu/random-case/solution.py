# https://www.codewars.com/kata/57073869924f34185100036d

# solution 1: 2586ms
import random
def random_case(x):
    return "".join(c.upper() if random.randint(0,255) % 2 == 1 else c.lower() for c in x)


# solution 2: 2078ms
import random 
def random_case(x):
    return "".join(random.choice([c.lower(), c.upper()]) for c in x)