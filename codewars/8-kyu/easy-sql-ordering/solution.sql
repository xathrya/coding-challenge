-- https://www.codewars.com/kata/593ed37c93350098d600001d
-- sort the information in the provided table 'companies' by number of employees (high to low).
-- Returned table should be in the same format as provided:

-- companies table schema
--    id
--    ceo
--    motto
--    employees

-- 3034ms
SELECT * FROM companies ORDER BY employees DESC 