-- https://www.codewars.com/kata/5811315e04adbbdb5000050e
-- create a SELECT statement, this statement must have NULL handling, using COALESCE and NULLIF
-- If name is an empty string, you must replace with '[product name not found]'.
-- If card_name is an empty string, you must replace with '[card name not found]'.
-- If no price is specified (i.e. price is NULL), or if the price is 50 or less, you must discard the row.

-- eusales table schema
--    id
--    name
--    price
--    card_name
--    card_number
--    transaction_date

-- resultant table schema
--    id
--    name
--    price (greater than 50.00)
--    card_name
--    card_number
--    transaction_date

-- solution 1: 1610ms
SELECT 
    id,
    COALESCE(NULLIF(name, ''), '[product name not found]') AS name,
    price,
    COALESCE(NULLIF(card_name, ''), '[card name not found]') AS card_name,
    card_number,
    transaction_date
FROM eusales 
WHERE price >= 50.0