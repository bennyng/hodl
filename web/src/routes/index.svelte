<script lang="ts">
  if (!import.meta.env.SSR) {
    const apiUrl = "https://api.hodl.commonlab-van.com/api/btc";
    // const apiUrl = "http://0.0.0.0:8000/api/btc";
    new EventSource(apiUrl).addEventListener(
      "message",
      (e) => (data = JSON.parse(e.data))
    );
  }

  let data: any;
  let tradeData = [];
  $: if (data) {
    tradeData = data?.marketUpdate?.tradesUpdate?.trades || [];
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
