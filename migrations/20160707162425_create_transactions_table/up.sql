-- TRANSACTIONS table -- Should probably add a company table and foreign key
-- to company on the credit, and remove company name from user tabl
CREATE TABLE IF NOT EXISTS transactions (
  id serial PRIMARY KEY,
  creditor_id integer NOT NULL,
  debtor_id integer NOT NULL,
  project_name VARCHAR NOT NULL,
  company_name VARCHAR NOT NULL,
  payment_number INTEGER NOT NULL,
  paid_date timestamp without time zone,
  created_date timestamp without time zone NOT NULL 
  );
-- TODO: Add foreign key constraint
