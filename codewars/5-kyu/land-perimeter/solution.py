# https://www.codewars.com/kata/5839c48f0cf94640a20001d3

# solution 1: 543ms
def land_perimeter(arr):
    total_perimeter = 0
    
    for idx_row in range(len(arr)):
        for idx_col in range(len(arr[idx_row])):
            if arr[idx_row][idx_col] == 'X':
                total_perimeter += 4
                if idx_col > 0 and arr[idx_row][idx_col-1] == 'X':
                    total_perimeter -= 2
                if idx_row > 0 and arr[idx_row-1][idx_col] == 'X':
                    total_perimeter -= 2
    
    return f"Total land perimeter: {total_perimeter}"

# 
land = lambda a: sum(t == ('X', 'X') for r in a for t in zip(r, r[1:])) * 2
def land_perimeter(arr):
    return "Total land perimeter: " + str("".join(arr).count("X") * 4 - land(arr) - land(zip(*arr)))