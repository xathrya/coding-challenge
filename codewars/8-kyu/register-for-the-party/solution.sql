-- https://www.codewars.com/kata/590cc86f7557c0494000007e
-- part #3 of Beginner SQL
-- insert statement to add yourself to the table of participants.
 
-- participants table schema
--    name (string)
--    age (integer)
--    attending (boolean)

-- solution 1: 1190ms
INSERT INTO participants (name, age, attending) VALUES ('xathrya', 31, TRUE);
SELECT * FROM participants;