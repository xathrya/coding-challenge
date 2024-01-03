# https://www.codewars.com/kata/5264d2b162488dc400000001

# solution 1: 501ms
def spin_words(sentence):
    return " ".join(map(lambda s: s[::-1] if len(s) >= 5 else s, sentence.split()))