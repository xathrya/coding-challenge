# https://www.codewars.com/kata/53af2b8861023f1d88000832

# solution 1: 643ms
def are_you_playing_banjo(name):
    return f"{name} {'plays' if name[0].lower()=='r' else 'does not play'} banjo"