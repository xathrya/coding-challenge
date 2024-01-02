# https://www.codewars.com/kata/55c933c115a8c426ac000082

# solution 1: 504ms
def eval_object(v):
    match v["operation"]:
        case "+":
            return v["a"] + v["b"]
        case "-":
            return v["a"] - v["b"]
        case "/":
            return v["a"] / v["b"]
        case "*":
            return v["a"] * v["b"]
        case "%":
            return v["a"] % v["b"]
        case "**":
            return v["a"] ** v["b"]
        case _:
            return 1

# solution 2: 540ms
def eval_object(v):
    return {
        "+":  v["a"] + v["b"],
        "-":  v["a"] - v["b"],
        "/":  v["a"] / v["b"],
        "*":  v["a"] * v["b"],
        "%":  v["a"] % v["b"],
        "**": v["a"] ** v["b"],
    }.get(v["operation"])


# solution 3: 514ms
def eval_object(v):
    return eval("{a}{operation}{b}".format(**v))