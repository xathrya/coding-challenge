-- https://www.codewars.com/kata/59440034e94fae05b2000073
-- select statement to return a single column table containing the full title of the person (concatenate all columns together except id)

-- names table schema
--    id
--    prefix
--    first
--    last
--    suffix

-- output table schema
--    title

-- solution 1: 1585ms
SELECT CONCAT(prefix, ' ', first, ' ', last, ' ', suffix) AS title FROM names 