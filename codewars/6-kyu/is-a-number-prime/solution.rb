# https://www.codewars.com/kata/5262119038c0985a5b00029f

def isPrime(num)
  num < 2 ? false : (2..num/2).none?{|i| num % i == 0}
end