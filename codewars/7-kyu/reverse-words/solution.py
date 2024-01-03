# https://www.codewars.com/kata/5259b20d6021e9e14c0010d4

# solution 1: 463ms
def reverse_words(text):
    return " ".join(s[::-1] for s in text.split(" "))