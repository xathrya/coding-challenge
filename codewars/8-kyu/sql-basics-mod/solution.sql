-- https://www.codewars.com/kata/594a9592704e4d21bc000131
-- return table with one column (mod) which is the output of number1 modulus number2

-- decimals table schema
--    id
--    number1
--    number2

-- solution 1: 1183ms
SELECT mod(number1,number2) AS mod FROM decimals