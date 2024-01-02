-- https://www.codewars.com/kata/5910b0d378cc2ba91400000b
-- part #4 Beginner SQL
-- Write a select statement to get a list of all students who haven't paid their tuition yet.
-- The list should include all the data available about these students.

-- students table schema
--    name (string)
--    age (integer)
--    semester (integer)
--    mentor (string)
--    tuition_received (Boolean)

-- solution 1
SELECT * FROM students WHERE tuition_received IS FALSE

-- solution 2
SELECT * FROM students WHERE tuition_received=FALSE

-- OPTION 3
SELECT * FROM students WHERE NOT tuition_received