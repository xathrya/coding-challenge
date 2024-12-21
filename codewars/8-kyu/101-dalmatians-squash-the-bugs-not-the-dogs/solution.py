# https://www.codewars.com/kata/56f6919a6b88de18ff000b36

# solution 1: 526ms
def how_many_dalmatians(n):
    dogs = ["Hardly any", "More than a handful!", "Woah that's a lot of dogs!", "101 DALMATIONS!!!"]
    return dogs[0] if n <= 10 else dogs[1] if n <= 50 else dogs[2] if n <= 100 else dogs[3]


# solution 2: 514ms
def how_many_dalmatians(n):
    LOOKUP = (
        (100,"101 DALMATIONS!!!"),
        (50, "Woah that's a lot of dogs!"),
        (10, "More than a handful!"), 
    )

    return next((msg for dogs,msg in LOOKUP if n > dogs), "Hardly any")


# solution 3: 522ms
def how_many_dalmatians(n):
    return {
        n == 101: "101 DALMATIONS!!!",
        n <= 50:  "More than a handful!",
        n <= 10:  "Hardly any",
    }.get(True, "Woah that's a lot of dogs!")