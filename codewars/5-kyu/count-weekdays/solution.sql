-- https://www.codewars.com/kata/58241d05e7a162c5b100010f
-- create a function that calculates the number of weekdays (Monday through Friday) between two dates inclusively.

-- the function:
--    weekdays(DATE, DATE) INTEGER
-- usage:
--    SELECT weekdays('2016-01-01', '2016-01-10')

-- solution 1: 1079ms
CREATE OR REPLACE FUNCTION weekdays(DATE, DATE) RETURNS INTEGER AS 
$$
    SELECT COUNT(days)::INTEGER 
    FROM generate_series(LEAST($1, $2), GREATEST($1, $2), '1 day') AS days 
    WHERE EXTRACT(DOW FROM days) NOT IN (0, 6);
$$
LANGUAGE SQL;