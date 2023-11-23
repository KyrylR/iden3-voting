-- Migration script to create 'commitments' table

-- Ensure the table doesn't already exist
DROP TABLE IF EXISTS commitments;

-- Create the 'commitments' table
CREATE TABLE commitments
(
    id           SERIAL PRIMARY KEY,
    proposal_id  INT          NOT NULL,
    commitment   VARCHAR(255) NOT NULL,
    block_number INT       NOT NULL
);