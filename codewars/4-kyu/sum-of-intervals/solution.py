# https://www.codewars.com/kata/52b7ed099cdc285c300001cd

# solution 1: 593ms
def sum_of_intervals(intervals: list) -> int:
    total=0
    intervals=sorted(intervals)
    a=intervals[0][0]
    b=intervals[0][1]
    total += b - a
    if len(set(intervals)) == 1:
        return total
    else:
        for j in intervals[1:]:
            if j[0] > b:
                total += j[1] - j[0]
                a = j[0]
                b = j[1]
            elif j[0] >= a and j[1] <= b:
                continue
            else:
                total += j[1] - b
                b=j[1]
    return total


# solution 2: 588ms
def sum_of_intervals(intervals):
    intervals.sort()
    end,subtract = intervals[0][1],0
    for i in intervals:
        if i[0] > end: subtract += i[0]-end
        if i[1] > end: end = i[1]
    return end - subtract - intervals[0][0]


# solution 3: 547ms
from bisect import bisect 
def sum_of_intervals(intervals):
    I = []
    for a,b in intervals:
        i = bisect(I, a)
        j = bisect(I, b)
        I = I[:i] + [a]*(1-i%2) + [b]*(1-j%2) + I[j:]
    
    return sum(I[1::2]) - sum(I[::2])