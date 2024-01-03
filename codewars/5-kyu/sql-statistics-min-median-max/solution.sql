-- https://www.codewars.com/kata/58167fa1f544130dcf000317
-- create a simple SELECT statement and calculate the MIN, MEDIAN and MAX scores of the students from the results table

-- result table schema
--    id
--    student_id
--    score

-- Resultant table:
--    min
--    median
--    max

-- option 1: 2014ms
SELECT MIN(score) AS min, PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY score) AS median, MAX(score) AS max FROM result 