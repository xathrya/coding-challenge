# https://www.codewars.com/kata/586560a639c5ab3a260000f3

# solution 1: 697ms
def rotate(str_):
    return [str_[i:]+str_[:i] for i in range(1,len(str_)+1)]