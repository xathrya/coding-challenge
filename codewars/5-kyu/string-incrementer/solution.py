# https://www.codewars.com/kata/54a91a4883a7de5d7800009c

# solution 1: 535ms
import re

def increment_string(text):
    if not text:
        return "1"
    number = re.search(r'\d+$', text)
    if not number:
        return text + "1"
    return text[:number.start()] + str(int(number.group()) + 1).zfill(len(number.group()))



# solution 2: 521ms
def increment_string(text):
    head = text.rstrip('0123456789')
    tail = text[len(head):]

    if len(tail) == 0:
        return text + "1"
    else:
        length = len(tail)
        new_tail = str(1 + int(tail)).zfill(length)
        return head + new_tail