-- https://www.codewars.com/kata/5811527d9d278b242f000006
-- create a VIEW of sales store to give out vouches to members who have spent over $1000 in 
-- departments that have brought in more than $10000 total ordered by the members id.
-- The VIEW must be called members_approved_for_voucher then you must create a SELECT query using the view.

-- resultant table schema
--    id
--    name
--    email
--    total_spending

-- option 1: 2750ms
CREATE VIEW members_approved_for_voucher AS 
SELECT members.id, members.name, members.email, SUM(products.price) AS total_spending
FROM members 
JOIN sales ON sales.member_id=members.id 
JOIN products ON products.id=sales.product_id 
WHERE sales.department_id IN (
    SELECT sales.department_id 
    FROM sales 
    INNER JOIN products ON products.id=sales.product_id 
    GROUP BY sales.department_id
    HAVING SUM(products.price) > 10000
)
GROUP BY members.id, members.name, members.email 
HAVING SUM(products.price) > 1000
ORDER BY members.id;

SELECT * FROM members_approved_for_voucher