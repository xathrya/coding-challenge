# https://www.codewars.com/kata/5dd462a573ee6d0014ce715b

# solution 1: 480ms
def same_case(a, b):
    if a.isalpha() and b.isalpha():
        if (a.isupper() and b.isupper()) or (a.islower() and b.islower()):
            return 1
        else:
            return 0
    else:
        return -1