# https://www.codewars.com/kata/583ade15666df5a64e000058

# solution 1: 548ms
def evens_and_odds(n):
    return (bin(n) if n % 2 == 0 else hex(n))[2:]