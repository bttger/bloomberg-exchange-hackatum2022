CREATE TABLE IF NOT EXISTS orders (
  id varchar
  timestamp bigint
  user_id varchar
  type smallint
  exec_type smallint
  symbol varchar
  amount int
  price int
);

CREATE TABLE IF NOT EXISTS trades (
  id varchar
  timestamp bigint
  user_id varchar
  symbol varchar
  amount int
  avg_price int
);
