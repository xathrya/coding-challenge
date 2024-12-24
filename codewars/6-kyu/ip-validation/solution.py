# https://www.codewars.com/kata/515decfd9dcfc23bb6000006

# solution 1: 529ms
def is_valid_IP(s):
    return s.count('.')==3 and all(o.isdigit() and 0<=int(o)<=255 and str(int(o))==o for o in s.split('.'))


# solution 2: 481ms
def is_valid_IP(address):
    octets = address.split(".")
    if len(octets) != 4:
        return False
    
    for part in octets:
        if not part.isdigit():
            return False
        num = int(part)
        if num < 0 or num > 255:
            return False
        if part != str(num):
            return False
    
    return True


# solution 3: 666ms
import re 

def is_valid_IP(address):
    return bool(re.match(r'(([1-9]?\d|1\d\d|2[0-4]\d|25[0-5])(\.(?!$)|$)){4}\Z', address))


# solution : 520ms
# this solution is cheating ^^
import socket

def is_valid_IP(address):
    try:
        socket.inet_pton(socket.AF_INET, address)
        return True
    except socket.error:
        return False