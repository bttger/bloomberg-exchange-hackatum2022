CREATE TABLE IF NOT EXISTS orders (
  id varchar,
  time_ bigint,
  user_id varchar,
  type_ smallint,
  exec_type smallint,
  symbol varchar,
  amount int,
  price int
);

CREATE TABLE IF NOT EXISTS trades (
  id varchar,
  time_ bigint,
  user_id varchar,
  symbol varchar,
  amount int,
  avg_price int
);
