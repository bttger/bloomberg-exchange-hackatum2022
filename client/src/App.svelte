<script>
  import Chart from "./lib/Chart.svelte";
  import FlexContainer from "./lib/FlexContainer.svelte";
  import Navbar from "./lib/Navbar.svelte";
  import TradeForm from "./lib/TradeForm.svelte";
  import AggOrderBook from "./lib/AggOrderBook.svelte";
  import { onMount } from "svelte";

  let aggOrderBook = {
    timestamp: 8274242333,
    ask: [
      {
        price: 9,
        amount: 20,
        total: 160,
      },
      {
        price: 8,
        amount: 20,
        total: 160,
      },
    ],
    bid: [
      {
        price: 7,
        amount: 20,
        total: 150,
      },
      {
        price: 5,
        amount: 20,
        total: 100,
      },
      {
        price: 4,
        amount: 10,
        total: 40,
      },
    ],
  };

  let trades = [
    {
      userId: "tom",
      symbol: "TWTR",
      timestamp: 1668876837,
      price: 8,
      amount: 20,
      total: 160,
    },
    {
      userId: "tom",
      symbol: "TWTR",
      timestamp: 1668886837,
      price: 9,
      amount: 20,
      total: 160,
    },
  ];

  function onCommand(event) {
    socket.send(event.detail);
  }

  let socket;

  onMount(() => {
    socket = new WebSocket("ws://127.0.0.1:3000/api");
    socket.addEventListener("message", (event) => {
      if (event.data.messageType === "aggOrderBook") {
        aggOrderBook = JSON.parse(event.data);
      }
      if (event.data.messageType === "trade") {
        trades.push(JSON.parse(event.data));
      }
    });
  });
</script>

<main>
  <Navbar />
  <FlexContainer>
    <AggOrderBook {aggOrderBook} />
    <Chart {trades} />
  </FlexContainer>
  <TradeForm on:command={onCommand} />
</main>
