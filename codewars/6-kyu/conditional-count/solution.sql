-- https://www.codewars.com/kata/5816a3ecf54413a113000074
-- produce a result set for the report that shows a side-by-side comparison of the number
-- and total amounts of payments made in Mike's and Jon's stores broken down by months.

-- solution 1: 1376ms
SELECT 
    EXTRACT(MONTH FROM payment_date) AS month,
    COUNT(DISTINCT payment_id) AS total_count,
    SUM(amount) AS total_amount,
    COUNT(CASE WHEN staff_id=1 THEN 1 ELSE NULL END) AS mike_count,
    SUM(CASE WHEN staff_id=1 THEN amount ELSE NULL END) AS mike_amount,
    COUNT(CASE WHEN staff_id=2 THEN 1 ELSE NULL END) AS jon_count,
    SUM(CASE WHEN staff_id=2 THEN amount ELSE NULL END) AS jon_amount
FROM payment 
GROUP BY month 