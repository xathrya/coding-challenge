# https://www.codewars.com/kata/585cf93f6ad5e0d9bf000010

# solution 1: 668ms
def bowling_pins(arr : list[int]) -> str:
    pins = tuple([" " if i in arr else "I" for i in [7, 8, 9, 10, 4, 5, 6, 2, 3, 1]])
    return "{} {} {} {}\n {} {} {} \n  {} {}  \n   {}   ".format(*pins)


# solution 2: 516ms
def bowling_pins(arr : list[int]) -> str:
    pins = ['I' for i in range(10)]
    for item in arr:
        pins[item-1] = ' '

    return "{6} {7} {8} {9}\n {3} {4} {5} \n  {1} {2}  \n   {0}   ".format(*pins)