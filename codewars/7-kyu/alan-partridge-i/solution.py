# https://www.codewars.com/kata/5808c8eff0ed4210de000008

# solution 1: 486ms
def part(arr):
    lut = ["Partridge","PearTree","Chat","Dan","Toblerone","Lynn", "AlphaPapa", "Nomad"]
    count = sum(arr.count(item) for item in lut)
    return f"Mine's a Pint{'!' * count}" if count > 0 else "Lynn, I've pierced my foot on a spike!!"
