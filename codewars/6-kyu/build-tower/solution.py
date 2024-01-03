# https://www.codewars.com/kata/576757b1df89ecf5bd00073b

# solution 1:
def tower_builder(n_floors):
    result = []
    for i in range(1, n_floors + 1):
        spaces = (n_floors - i)
        result.append(" " * spaces + "*" * (i*2 - 1) + " " * spaces)
    return result


# solution 2:
def tower_builder(n_floors):
    return [("*" * (i*2-1)).center(n*2-1) for i in range(1, n_floors + 1)]