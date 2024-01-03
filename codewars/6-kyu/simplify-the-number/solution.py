# https://www.codewars.com/kata/5800b6568f7ddad2c10000ae/

# solution 1: 480ms
def simplify(number):
    number = str(number)
    if len(number) == 1:
        if number == "0":
            return ""
        else:
            return number 

    l = len(number) - 1
    result = []

    for c in number[:-1]:
        if c != "0":
            result.append(f"{c}*1{'0'*l}")
        l -= 1
    
    if number[-1] != "0":
        result.append(number[-1])
    
    return "+".join(result)