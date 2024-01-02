# https://www.codewars.com/kata/54147087d5c2ebe4f1000805

# solution 1: 510ms
def _if(bool, func1, func2):
    if bool:
        func1()
    else:
        func2()