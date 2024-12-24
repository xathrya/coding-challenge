# https://www.codewars.com/kata/66fc066f4339d4d65fe7d0bf

# solution 1: 477ms
fizznotbuzz = lambda s:'fizz'in s==s-{'buzz'}

# solution 2: 626ms
fizznotbuzz = lambda s:'fizz'in s<s|{'buzz'}

# solution 3: 512ms
fizznotbuzz = lambda s:{'fizz'}^s<{'buzz'}^s

# solution 4: 500ms
fizznotbuzz = lambda s:s-{'buzz'}==s|{'fizz'}

# solution 5: 513ms
fizznotbuzz = lambda s:s&{'fizz'}>s&{'buzz'}

# not solution due to characters length constraint
# option 1: 47 bytes
fizznotbuzz = lambda s:(s-{"buzz"}|{"fizz"})==s 