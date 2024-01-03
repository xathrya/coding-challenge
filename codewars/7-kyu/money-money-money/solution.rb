# https://www.codewars.com/kata/563f037412e5ada593000114

# solution 1: 922ms
def calculate_years(principal, interest, tax, desired)
    years=0
    money = principal
    while money < desired do
      money += money * interest * (1-tax)
      years += 1
    end
    years
  end