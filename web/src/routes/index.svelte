<script lang="ts">
  if (!import.meta.env.SSR) {
    const sse = new EventSource(`http://0.0.0.0:8000/api/btc`);

    sse.addEventListener("message", async (e) => {
      data = JSON.parse(e.data);
      console.log("data", data);
    });
  }

  let data: any;
  let tradeData = [];
  $: if (data) {
    tradeData = data?.marketUpdate?.tradesUpdate?.trades || [];
    console.log("tradeData", tradeData);
  }
</script>

<div class="row">
  <h1>btc trades</h1>
  {#if tradeData && tradeData.length > 0}
    {#each tradeData as trade (trade.externalId)}
      {trade.amountQuoteStr}@{trade.priceStr}
      <br />
    {/each}
  {:else}
    connecting...
  {/if}
</div>

<div class="row" />
