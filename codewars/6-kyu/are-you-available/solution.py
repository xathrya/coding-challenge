# https://www.codewars.com/kata/5603002927a683441f0000cb

# solution 1: 519ms
def check_availability(schedule, current_time):
    for start, finish in schedule:
        if int(start[:2]) <= int(current_time[:2]) <= int(finish[:2]) and int(current_time[3:]) < int(finish[3:]):
            return t[1]
    
    return True