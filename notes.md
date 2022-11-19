# Tasks

For now we will hard-code the security symbol that clients trade on the exchange. Though the database schema will support many securities. Another design decision is that we don't allow fractional buys/sells and prices must be integers.

- order book server (OBS) starts
  - OBS subscribes to trade channel
  - -"- subscribes to order channel
  - -"- listens on socket
- matching server (MS) starts
  - MS subscribes to order channel
  - on new order event, check if trade is possible, and if so, update DB and publish new event on trading channel
- client connects to socket
  - server sends unique user ID maintained for the socket session (makes it simpler for us so we don't need to implement authentication)
  - -"- SELECTs and forwards latest aggregated order book and last n trades from relational DB (e.g. Postgres with Timescale extension; throttled/cached to 1 req/X ms)
  - -"- forwards all events from the trade channel
  - -"- on event in order channel, SELECT latest aggregated order book and forward to all clients (throttled/cached to 1 req/X ms)
  - -"- listens for order commands (add immediate or limit order; delete order)
- client sends order command with his ID
  - OBS updates DB
  - -"- publishes event in order channel (which the matching server and all clients are subscribed to)

# Stack

## Server
- Rust, Axum for WebSocket handling
- PostgreSQL with Timescale extension (for continuous aggregates)
- Redis as our Pub/Sub broker

## Client
- Svelte web app with WebSockets

# Tables

## Orders
```
id varchar (uuid or nano id, assigned by OBS)
timestamp bigint (unix timestamp)
user_id varchar (users send it with commands)
type smallint ('bid'=0, 'ask'=1)
exec_type smallint ('market'=0, 'limit'=1)
symbol varchar
amount int
price int (optional, only for limit orders)
```

## Trades
```
id varchar (uuid or nano id, assigned by MS)
timestamp bigint (unix timestamp)
user_id varchar
symbol varchar
amount int
avg_price double (when multiple entries from order book needed to fulfil trade)
```

## Schema
```sql
CREATE TABLE IF NOT EXISTS orders (
  id varchar PRIMARY KEY,
  time_ bigint,
  user_id varchar,
  type_ smallint,
  exec_type smallint,
  symbol varchar,
  amount int,
  price int
);

CREATE TABLE IF NOT EXISTS trades (
  id varchar PRIMARY KEY,
  time_ bigint,
  user_id varchar,
  symbol varchar,
  amount int,
  avg_price double
);
```

## Queries

**addOrder(args[])** (executed by OBS)
```sql
INSERT INTO orders VALUES (id, time_, user_id, type_, exec_type, symbol, amount, price);
```

**addTrade(args[])** (executed by MS)
```sql
INSERT INTO trades VALUES (id, time_, user_id, symbol, amount, avg_price);
```

**getLatestTrades(number int)** (only queried on client init and trade event by OBS)
```sql
SELECT time_, user_id, amount, avg_price FROM trades WHERE symbol = $symbol ORDER BY time_ DESC LIMIT $number;
```

**getAggOrderBook(symbol string)** (only queried on client init and order event by OBS)
```sql
SELECT price, sum(amount) AS total_amount, total_amount * price AS total_price
FROM orders
WHERE type_ = 0 AND exec_type = 1 AND symbol = $symbol
GROUP BY price
ORDER BY price DESC;

SELECT price, sum(amount) AS total_amount, total_amount * price AS total_price
FROM orders
WHERE type_ = 1 AND exec_type = 1 AND symbol = $symbol
GROUP BY price
ORDER BY price DESC;
```

**getMatchingOrders(symbol string, type_ smallint, amount int, optional price int)** (only queried on new order event by MS)

market order (we don't save the order in the DB but immediately execute it with the closest possible buy/sell orders; what if there are not enough items to trade on the exchange?):
```sql
SELECT *
FROM orders
WHERE CASE
  WHEN $type_ = 0
  THEN type_ = 1
  ELSE type_ = 0
END AND symbol = $symbol
ORDER BY time_ ASC;
```

limit order (we get all buy orders that overlap in price with sell orders, and then execute them by FIFO principle until the amount of the sell orders is exhausted; the following query must be executed multiple times if the amounts of the lowest ask price is not sufficient to fulfill bid order):
```sql
ask 30
ask 29
ask 28 300
ask 27 100 # amount is not sufficient
bid 28 300
bid 26
bid 23

SELECT *
FROM orders
WHERE type_ = 0 AND symbol = $symbol AND price >= (
  SELECT min(price)
  FROM orders
  WHERE type_ = 1 AND symbol = $symbol
  )
ORDER BY time_ ASC;
```



# API

- Single websocket endpoint (server view)
  - onOpen: OBS, initializes connection with new user id, sends aggregated order book and last n trades
  - onMessage: OBS checks and executes command (add, delete)
  - onClose: clean up connection, delete user ID

## Structs

```
aggOrderBook: {
  timestamp int,
  ask: [{
      price float
      amount float
      total float
  }],
  bid: [{
      price float
      amount float
      total float
  }]
}

trades: [{
    userId string
    security string
    timestamp int
    price float
    amount float
    total float
}]

```
