# https://www.codewars.com/kata/5208f99aee097e6552000148

# solution 1: 500ms
def solution(s):
    if s == "":
        return s
    
    result = []
    last_idx = 0
    for idx in range(len(s)):
        if s[idx].isupper():
            result.append(s[last_idx:idx])
            last_idx = idx
    
    result.append(s[last_idx:])
    
    return " ".join(result)


# solution 2:
def solution(s):
    result = ""
    for c in s:
        if c.isupper():
            result += " "
        result += c 
    
    return result 


# solution 3:
def solution(s):
    return "".join(c if c.islower() else " " + c for c in s)