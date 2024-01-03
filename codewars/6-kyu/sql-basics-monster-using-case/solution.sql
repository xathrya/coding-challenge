-- https://www.codewars.com/kata/593ef0e98b90525e090000b9
-- return table with specific format

-- top_half schema
--     id
--     heads
--     arms

-- bottom_half schema
--     id
--     legs
--     tails

-- output schema
--    id
--    heads
--    legs
--    arms
--    tails
--    species

-- solution 1: 2521ms
SELECT top_half.id, heads, bottom_half.legs, arms, bottom_half.tails,
    CASE WHEN heads > arms OR bottom_half.tails > bottom_half.legs THEN 'BEAST' ELSE 'WEIRDO'
    END AS species 
from top_half join bottom_half on top_half.id=bottom_half.id 
ORDER BY species 