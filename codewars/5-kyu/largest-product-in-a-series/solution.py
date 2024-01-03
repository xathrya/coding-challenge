# https://www.codewars.com/kata/529872bdd0f550a06b00026e/

# solution 1: 527ms
def greatest_product(st):
    maxval = 0
    for i in range(len(st)+1):
        prod = 1
        for x in st[i:i+5]:
            if len(st[i:i+5]) < 5 or "0" in st[i:i+5]:
                break 
            prod = prod * int(x)
        if prod > maxval and prod != 1:
            maxval = prod 
    
    return maxval 