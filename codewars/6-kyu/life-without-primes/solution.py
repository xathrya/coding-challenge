# https://www.codewars.com/kata/59f8750ac374cba8f0000033

# solution 1: 8043ms
def solve(n):
    if n == 0:
        return 1
    
    arr = []
    idx = 1

    is_prime_number = lambda number: all( number%i != 0 for i in range(2, int(number**.5)+1) )
    
    while len(arr) < n:
        if not is_prime_number(idx) and not is_prime_digit(idx):
            arr.append(idx)
        idx += 1
    
    return arr[-1]


def is_prime_digit(number):
    number = str(number)
    for pd in ['2', '3', '5', '7']:
        if pd in number:
            return True 
    
    return False


# solution 2: 681ms
n, ban = 100000, set("2357")
sieve, target = [0]*(n+1), [1]
for i in range(2, n+1):
    if sieve[i]:
        if not (ban & set(str(i))):
            target.append(i)
    else:
        for j in range(i**2, n+1, i):
            sieve[j] = 1

def solve(n):
    return target[n]