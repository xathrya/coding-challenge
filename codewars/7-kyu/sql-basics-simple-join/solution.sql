-- https://www.codewars.com/kata/5802e32dd8c944e562000020
-- create a simple SELECT statement that will return all columns from the products table,
-- and join to the companies table so that you can return the company name. 

-- products table schema
--     id
--     name
--     isbn
--     company_id
--     price

-- companies table schema
--     id
--     name

-- solution 1: 3669ms
SELECT p.id, p.name, p.isbn, p.company_id, p.price, c.name AS company_name 
FROM products AS p INNER JOIN companies AS c on p.company_id=c.id