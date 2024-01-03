# https://www.codewars.com/kata/530e15517bc88ac656000716

# solution 1: 564ms
import string as st

indexes = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
lookups = "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm"

def rot13(message):
    rule = str.maketrans(indexes, lookups)
    return message.translate(rule)

def rot13(message):
    result = []
    for c in message:
        if c in indexes:
            result.append(lookups[indexes.index(c)])
        else:
            result.append(c)
    
    return "".join(result)
