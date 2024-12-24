# https://www.codewars.com/kata/57a083a57cb1f31db7000028
# might be related to https://www.codewars.com/kata/534d0a229345375d520006a0

# solution 1: 510ms
def powers_of_two(n):
    return [1 << i for i in range(n+1)]


# solution 2: 713ms
def powers_of_two(n):
    return [2**i for i in range(n+1)]


# solution 3: 554ms
def powers_of_two(n):
    result = [1]
    for i in range(1, n+1):
        result.append(result[i-1] * 2)
    
    return result