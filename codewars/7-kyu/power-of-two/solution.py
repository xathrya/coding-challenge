# https://www.codewars.com/kata/534d0a229345375d520006a0
# might be related to https://www.codewars.com/kata/57a083a57cb1f31db7000028

# solution 1: 480ms
def power_of_two(x):
    if x <= 0: return False
    return (x & (x - 1)) == 0


# solution 2: 491ms
def power_of_two(x): 
    return x != 0 and (x & (x - 1)) == 0


# solution 3: 504ms
def power_of_two(x):
    return bin(x).count('1') == 1


# solution 4: 471ms
def power_of_two(x):
    while x > 1 and x % 2 == 0:
        x = x // 2 
    
    return x == 1