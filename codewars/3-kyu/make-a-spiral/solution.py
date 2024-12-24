# https://www.codewars.com/kata/534e01fbbb17187c7e0000c6

# solution 1: 560ms
def spiralize(size):
    board = [[0 for x in range(size)] for y in range(size)]
    x, y = -1, 0
    dx, dy = 1, 0
    turns = 0

    on_board = lambda x,y: 0 <= x < size and 0 <= y < size 
    is_one   = lambda x,y: on_board(x,y) and board[y][x] == 1 
    can_move = lambda: on_board(x+dx, y+dy) and not (is_one(x+2*dx, y+2*dy) or is_one(x+dx-dy, y+dy+dx) or is_one(x+dx+dy, y+dy-dx))

    while (turns < 2):
        if can_move():
            x += dx 
            y += dy
            board[y][x] = 1
            turns = 0
        else:
            dx, dy = -dy, dx 
            turns += 1
    
    return board


# solution 2: 767ms
import numpy as np
def spiralize(size):
    if size == 0: return []
    if size == 1: return [[1]]
    if size == 2: return [[1,1], [0,1]]

    board = np.zeros((size, size))
    board[0, :] = 1
    board[:, -1] = 1
    board[-1, :] = 1
    board[2:, :-2] = np.array(spiralize(size-2))[::-1, ::-1]
    return board.tolist()


# solution 3: 565ms
def spiralize(size):
    return [[
        min(i + 1, size - i, j + 1, size - j) & 1 ^ (i == j + 1 and (j if size % 4 == 2 else i) * 2 < size)
        for j in range(size)]
        for i in range(size)]