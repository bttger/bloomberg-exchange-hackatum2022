<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  let username = "";
  let exec_type = 0;
  let amount;
  let price;

  function sendCommand(type) {
    dispatch("command", { type, username, exec_type, amount, price });
    price = undefined;
    amount = undefined;
  }
</script>

<div class="container">
  {#if username}
    <h4>Logged in as <span style="font-style:italic">{username}</span></h4>
    <div class="flex">
      <button class:selected={exec_type === 0} on:click={() => (exec_type = 0)}
        >Market Order</button
      >
      <button class:selected={exec_type === 1} on:click={() => (exec_type = 1)}
        >Limit Order</button
      >
    </div>
    <div class="flex">
      <label for="price">Price</label>
      <input
        id="price"
        type="number"
        placeholder="Price"
        bind:value={price}
        disabled={exec_type === 0 || null}
      />
      <label for="quantity">Quantity</label>
      <input
        id="quantity"
        type="number"
        placeholder="Quantity"
        bind:value={amount}
      />
    </div>
    <div class="flex">
      <button class="buy-button" on:click={() => sendCommand(0)}>Buy</button>
      <button class="sell-button" on:click={() => sendCommand(1)}>Sell</button>
    </div>
  {:else}
    <h4>Please enter a username to start trading</h4>
    <input id="username" type="text" placeholder="Username" />
    <button
      on:click={() => (username = document.getElementById("username").value)}
      >Start Trading</button
    >
  {/if}
</div>

<style>
  .container {
    margin-top: 20pt;
  }
  .flex {
    display: flex;
    gap: 10pt;
  }

  .selected {
    background-color: rgb(61, 105, 224);
    border-color: rgb(61, 105, 224);
    color: white;
  }

  .selected:hover {
    background-color: rgb(51, 98, 226);
    border-color: rgb(61, 105, 224);
    color: rgb(230, 230, 230);
  }

  .buy-button {
    background-color: rgb(2, 161, 10);
    border-color: rgb(2, 161, 10);
    color: white;
  }

  .buy-button:hover {
    background-color: rgb(2, 136, 9);
    border-color: rgb(2, 161, 10);
    color: rgb(230, 230, 230);
  }

  .sell-button {
    background-color: rgb(160, 2, 63);
    border-color: rgb(160, 2, 63);
    color: white;
  }

  .sell-button:hover {
    background-color: rgb(141, 1, 54);
    border-color: rgb(160, 2, 63);
    color: rgb(230, 230, 230);
  }

  input:disabled {
    background-color: rgb(207, 207, 207);
  }
</style>
