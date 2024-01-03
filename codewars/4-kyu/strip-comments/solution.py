# https://www.codewars.com/kata/51c8e37cee245da6b40000bd/

# solution 1: 499ms
def strip_comments(strng, markers):
    s_list = strng.split('\n')
    for marker in markers:
        s_list = [item.split(marker)[0].rstrip() for item in s_list]
    return '\n'.join(s_list)

