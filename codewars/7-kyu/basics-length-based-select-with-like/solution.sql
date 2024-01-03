-- https://www.codewars.com/kata/5a8d94d3ba1bb569e5000198
-- create SELECT statement in conjunction with LIKE
-- list people which have first_name with at least 6 character long

-- names table schema
--     id
--     first_name
--     last_name

-- results table schema
--     first_name
--     last_name

-- solution 1: 2665ms
SELECT first_name,last_name FROM names WHERE first_name LIKE '______%'

-- alternative
SELECT first_name,last_name FROM names WHERE first_name ~ '\W{6}'