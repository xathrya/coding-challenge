# https://www.codewars.com/kata/5526fc09a1bbd946250002dc

def find_outlier(integers)
    even,odd = integers.partition {|v| v.even? }
    if even.length == 1 
      return even[0]
    elsif odd.length == 1
      return odd[0]
    end
end