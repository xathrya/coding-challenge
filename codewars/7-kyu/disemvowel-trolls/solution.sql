-- https://www.codewars.com/kata/52fba66badcd10859f00097e
-- return a table with column 'str' and your result in a column named 'res'.
-- you are given a table 'disemvowel' with column 'str', 

-- solution 1: 1140ms
SELECT str,TRANSLATE(str, 'aeiouAEIOU', '') AS res FROM disemvowel

-- solution 2: 1540ms
SELECT str,REGEXP_REPLACE(str, '[aeiou]', '', 'ig') AS res FROM disemvowel