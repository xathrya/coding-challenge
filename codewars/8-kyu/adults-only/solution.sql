-- https://www.codewars.com/kata/590a95eede09f87472000213
-- part #1 Beginner SQL

-- get list of names and ages of users from the users table who are 18 years or older

-- users table schema
--    name
--    age

-- solution 1: 1543ms
SELECT name,age FROM users WHERE age >= 18