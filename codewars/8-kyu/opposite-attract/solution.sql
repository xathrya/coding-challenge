-- https://www.codewars.com/kata/555086d53eac039a2a000083

-- write your SQL statement here: you are given a table 'love' with columns 'flower1' and 'flower2', 
-- return a table with all the columns and your result in a column named 'res'.

-- solution 1: 1117ms
SELECT flower1, flower2, (flower1 % 2 = 0 AND flower2 % 2 != 0) OR (flower2 % 2 = 0 AND flower1 % 2 != 0) AS res
FROM love;