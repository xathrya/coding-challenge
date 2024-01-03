-- https://www.codewars.com/kata/5e5f09dc0a17be0023920f6f
-- part #6 Beginner SQL

-- Question: from the African countries that start with the character E, get the names of their capitals ordered alphabetically.
--  limit to max 3 results
--  country name can be Africa and Afrika

-- Schema of the countries table:
--     country (String)
--     capital (String)
--     continent (String)

-- solution 1: 1315ms
SELECT capital FROM countries WHERE continent IN ('Africa','Afrika') AND country LIKE 'E%' ORDER BY capital ASC LIMIT 3