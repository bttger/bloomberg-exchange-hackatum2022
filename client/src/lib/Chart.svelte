<script>
  import { createChart } from "lightweight-charts";
  import { onMount } from "svelte";

  let container;
  let chart;
  let lineSeries;
  let mounted = false;

  export let trades;

  onMount(() => {
    chart = createChart(container);
    lineSeries = chart.addLineSeries();
    mounted = true;
  });

  $: {
    if (mounted) {
      const transformedTrades = trades.map((t) => ({
        time: t.timestamp,
        value: t.price,
      }));
      console.log(transformedTrades);
      lineSeries.setData(transformedTrades);
      chart.timeScale().fitContent();
    }
  }
</script>

<div bind:this={container} />

<style>
  div {
    width: 70vw;
    height: 70vh;
  }
</style>
