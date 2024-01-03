# https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa

# solution 1: 530ms
def selection(candidate):
    age, handicap = candidate
    if age >= 55 and handicap > 7:
        return "Senior"
    else:
        return "Open"

def open_or_senior(data):
    return [selection(c) for c in data]


# solution 2: 
def open_or_senior(data):
    result = []
    for candidate in data:
        age, handicap = candidate
        if age >= 55 and handicap > 7:
            result.append("Senior")
        else:
            result.append("Open")
    
    return result 


# solution 3: 
def open_or_senior(data):
    return ["Senior" if c[0]>=55 and c[1]>7 else "Open" for c in data]