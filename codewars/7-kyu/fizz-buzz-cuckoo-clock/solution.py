# https://www.codewars.com/kata/58485a43d750d23bad0000e6

# solution 1: 517ms
def fizz_buzz_cuckoo_clock(time):
    hh, mm = map(int, time.split(":"))
    
    result = ""
    if mm == 0:
        result = " ".join(["Cuckoo"] * (hh % 12 or 12))
    elif mm == 30:
        result = "Cuckoo"
    elif mm % 15 == 0:
        result = "Fizz Buzz"
    elif mm % 3 == 0:
        result = "Fizz"
    elif mm % 5 == 0:
        result = "Buzz"
    else:
        result = "tick"
    
    return result