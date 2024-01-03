# https://www.codewars.com/kata/52bc74d4ac05d0945d00054e/

# solution 1: 493ms
def first_non_repeating_letter(s):
    reps = [c for c in s if s.lower().count(c.lower()) == 1]
    return  reps[0] if len(reps) > 0 else ""