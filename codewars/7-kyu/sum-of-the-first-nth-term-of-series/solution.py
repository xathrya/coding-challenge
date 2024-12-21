# https://www.codewars.com/kata/555eded1ad94b00403000071

# solution 1: 501ms
def series_sum(n):
    result = 0
    i = 0
    while i < n:
        result += 1/(1+i*3)
        i += 1
    
    return f"{result:.2f}"


# solution 2: 
def series_sum(n):
    return "{:.2f}".format(sum(1.0/(3 * i + 1) for i in range(n)))