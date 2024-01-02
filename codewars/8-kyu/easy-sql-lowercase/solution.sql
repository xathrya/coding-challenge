-- https://www.codewars.com/kata/594800ba6fb152624300006d
-- return the same table where all letters are lowercase in the race column 

-- demographics table schema
--    id
--    name
--    birthday
--    race

-- solution 1: 2838ms
SELECT id,name,birthday,LOWER(race) AS race FROM demographics