-- https://www.codewars.com/kata/591127cbe8b9fb05bd00004b
-- part #5 of Beginner SQL
-- Use a select statement to list names, authors, and number of copies sold of the 5 books
-- which were sold most.

-- books table schema
--    name
--    author
--    copies_sold

-- solution 1: 1165ms
SELECT name,author,copies_sold FROM books ORDER BY copies_sold DESC LIMIT 5 