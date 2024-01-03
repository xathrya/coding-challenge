# https://www.codewars.com/kata/556deca17c58da83c00002db

def tribonacci(signature,n)
  n < 3 ? signature[0, n] : (n-3).times.each_with_object(signature) { |num, obj| obj << obj[-3] + obj[-2] + obj[-1] }
end