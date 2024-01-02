# https://www.codewars.com/kata/57356c55867b9b7a60000bd7/

# solution 1: 494ms
def basic_op(operator, value1, value2):
    return {
        "+": value1 + value2,
        "-": value1 - value2,
        "*": value1 * value2,
        "/": value1 / value2,
    }[operator]


# solution 2: 519ms
def basic_op(operator, value1, value2):
    return eval(f"{value1}{operator}{value2}")


# solution 3:
def basic_op(operator, value1, value2):
    match operator:
        case "+":
            return value1 + value2
        case "-":
            return value1 - value2
        case "*":
            return value1 * value2
        case "/":
            return value1 / value2