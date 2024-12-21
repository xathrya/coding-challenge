# https://www.codewars.com/kata/545cedaa9943f7fe7b000048

# solution 1: 499ms
import string
def is_pangram(st):
    return set(st.lower()) >= set(string.ascii_lowercase)


# solution 2:
import string 
def is_pangram(st):
    return set(string.ascii_lowercase).issubset(set(st.lower()))