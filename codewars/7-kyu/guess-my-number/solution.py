# https://www.codewars.com/kata/5654d2428be803670a000030

# solution 1: 475ms
def guess_my_number(guess, number = '123-451-2345'):
    mask = guess + "-"
    return "".join(c if c in mask else "#" for c in number)