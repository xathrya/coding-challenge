# https://www.codewars.com/kata/546f922b54af40e1e90001da

# solution 1: 540ms
def alphabet_position(text):
    return " ".join(str(ord(c) - 0x60) for c in filter(str.isalpha, text.lower()))
