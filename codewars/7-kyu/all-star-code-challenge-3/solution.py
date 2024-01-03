# https://www.codewars.com/kata/58640340b3a675d9a70000b9

# solution 1: 472ms
def remove_vowels(strng):
    for vowel in ["a", "e", "i", "o", "u"]:
        strng = strng.replace(vowel, "")
    
    return strng


# solution 2: 533ms
def remove_vowels(strng):
    return "".join(c for c in strng if c not in "aeiou")