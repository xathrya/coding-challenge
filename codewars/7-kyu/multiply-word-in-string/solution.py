# https://www.codewars.com/kata/5ace2d9f307eb29430000092

# solution 1: 653ms
def modify_multiply(st, loc, num):
    return "-".join([st.split()[loc]] * num)