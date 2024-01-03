# https://www.codewars.com/kata/517abf86da9663f1d2000003

# solution 1: 490ms
import re 
def to_camel_case(text):
    words = re.split('-|_', text)
    return words[0] + "".join(w.capitalize() for w in words[1:])