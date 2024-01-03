# https://www.codewars.com/kata/514b92a657cdc65150000006

# solution 1: 887ms
def solution(number)
  (1...number).select{|n| (n % 5).zero? || (n % 3).zero?}.reduce(:+)
end