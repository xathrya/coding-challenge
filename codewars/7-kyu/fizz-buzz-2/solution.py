# https://www.codewars.com/kata/5300901726d12b80e8000498
# # There are multiple kata with Fizz Buzz name

# solution 1: 678ms
def fizzbuzz(n):
    result = []
    for i in range(1, n+1):
        if i % 15 == 0:
            result.append("FizzBuzz")
        elif i % 3 == 0:
            result.append("Fizz")
        elif i % 5 == 0:
            result.append("Buzz")
        else:
            result.append(i)
    
    return result


# solution 2: 518ms
def fizzbuzz(n):
    return [fb(m) for m in range(1,n+1)]

def fb(m):
    r = (m % 3 == 0) * "Fizz" + (m % 5 == 0) * "Buzz"
    return r if r != "" else m