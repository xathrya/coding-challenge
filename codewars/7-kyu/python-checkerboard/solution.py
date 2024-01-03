# https://www.codewars.com/kata/57785441311a24465e000025

# solution 1: 709ms
def make_checkered_board(n):
    board = []
    for x in range(0,n):
        temp = []
        for y in range(0,n):
            temp.append("X") if (x+y) % 2 == 0 else temp.append("O")
        board.append(temp)
    return board