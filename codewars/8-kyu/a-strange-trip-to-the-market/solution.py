# https://www.codewars.com/kata/a-strange-trip-to-the-market

# solution 1: 493ms
def is_loch_ness_monster(s):
    return "tree fiddy" in s or "3.50" in s or "three fifty" in s


# solution 2: 476ms
def is_loch_ness_monster(s):
    return any(i in s for i in ('tree fiddy', '3.50', 'three fifty'))


# solution 3: 548ms
import re
def is_loch_ness_monster(s):
    return bool(re.search(r'tree fiddy|3\.50|three fifty', s))