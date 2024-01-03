# https://www.codewars.com/kata/5467e4d82edf8bbf40000155

# solution 1: 971ms
def descending_order(n)
  n.to_s.split('').sort.reverse!.join.to_i
end