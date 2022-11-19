<script>
  export let aggOrderBook;
</script>

<div class="container">
  <p>Last update: {new Date(aggOrderBook.timestamp).toLocaleTimeString()}</p>
  <table>
    <thead>
      <tr>
        <th>Price</th>
        <th>Quantity</th>
        <th>Total</th>
      </tr>
    </thead>
    <tbody>
      {#each aggOrderBook.ask as ask, i}
        <tr class="ask-rows">
          <td>{ask.price}</td>
          <td>{ask.amount}</td>
          <td>{ask.total}</td>
        </tr>
      {/each}

      {#each aggOrderBook.bid as ask, i}
        <tr class="bid-rows">
          <td>{ask.price}</td>
          <td>{ask.amount}</td>
          <td>{ask.total}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .container {
    margin-right: 20pt;
  }

  .ask-rows {
    color: darkred;
    background-color: rgb(255, 194, 194);
  }

  .bid-rows {
    color: darkgreen;
    background-color: rgb(188, 255, 191);
  }

  table {
    max-height: 70vh;
    overflow-y: auto;
  }

  thead {
    top: 0;
    position: sticky;
    background-color: white;
  }
</style>
