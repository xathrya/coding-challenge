# https://www.codewars.com/kata/5842df8ccbd22792a4000245

# solution 1: 508ms
def expanded_form(num):
    num = list(str(num))
    return " + ".join(d + '0' * (len(num) - i - 1) for i, d in enumerate(num) if d != '0')

# solution 2: 602ms
def expanded_form(num):
    pos = 1
    result = []
    while num > 0:
        unit = num % 10
        if unit != 0:
            result.append(str((num % 10) * pos))
        pos *= 10
        num = num // 10
    
    return " + ".join(result[::-1])