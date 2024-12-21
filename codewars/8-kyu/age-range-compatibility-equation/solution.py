# https://www.codewars.com/kata/5803956ddb07c5c74200144e

# solution 1: 490ms
def dating_range(age):
    if age >= 14:
        age_min = age // 2 + 7
        age_max = (age - 7) * 2
    else:
        age_min = int(age - 0.10 * age)
        age_max = int(age + 0.10 * age)
    return f"{age_min}-{age_max}"
