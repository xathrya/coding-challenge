# https://www.codewars.com/kata/576757b1df89ecf5bd00073b

# solution 1: 514ms
def tower_builder(n_floors):
    return [("*" * (i*2-1)).center(n_floors*2-1) for i in range(1, n_floors+1)]

# solution 2: 526ms
def tower_builder(n_floors):
    return ["{:^{}}".format("*" * i, n_floors*2 - 1) for i in range(1, n_floors*2, 2)]

# solution 3: 521ms
def tower_builder(n_floors):
    return [" " * (n_floors - i - 1) + "*" * (2*i + 1) + " " * (n_floors - i - 1) for i in range(n_floors)]

# solution 4: 545ms
def tower_builder(n_floors):
    result = []
    for i in range(1, n_floors + 1):
        stars  = "*" * (i * 2 - 1)
        spaces = " " * (n_floors - i)
        result.append(spaces + stars + spaces)
    return result