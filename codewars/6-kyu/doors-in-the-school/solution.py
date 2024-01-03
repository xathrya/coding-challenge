# https://www.codewars.com/kata/57c15d314677bb2bd4000017

# solution 1: 12072ms 
def doors(n):
    doors = [True] * n 
    for i in range(1, n):
        for j in range(i, n, i+1):
            doors[j] = not doors[j]
    
    return doors.count(True)