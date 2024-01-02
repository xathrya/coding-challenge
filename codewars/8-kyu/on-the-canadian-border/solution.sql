-- https://www.codewars.com/kata/590ba881fe13cfdcc20001b4
-- part #2 Beginner SQL

-- select names and countries of origin of all travelers excluding anyone from Canada, Mexico or USA

-- travelers table schema
--    name
--    country

-- solution 1: 1202ms
SELECT name,country FROM travelers WHERE country NOT IN ('Canada','Mexico','USA')