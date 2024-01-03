# https://www.codewars.com/kata/5266876b8f4bf2da9b000362

# solution 1: 511ms
def likes(names):
    if len(names) == 0:
        return "no one likes this"
    elif len(names) == 1:
        return f"{names[0]} likes this"
    elif len(names) == 2:
        return f"{names[0]} and {names[1]} like this"
    else:
        who = ", ".join(names[:2])
        if len(names) == 3:
            who += f" and {names[2]}"
        else:
            who += f" and {len(names) - 2} others"
        return f"{who} like this"