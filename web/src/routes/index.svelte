<script lang="ts">
  if (!import.meta.env.SSR) {
    const apiUrl =
      import.meta.env.PROD === true
        ? "https://api.hodl.commonlab-van.com/api/btc"
        : "http://0.0.0.0:8000/api/btc";

    new EventSource(apiUrl).addEventListener(
      "message",
      (e) => (data = JSON.parse(e.data))
    );
  }

  let data: any;
  let tradeData = [];
  $: if (data) {
    tradeData = data?.data || [];
  }
</script>

<div class="row">
  <h1>btc trades</h1>

  {#if tradeData && tradeData.length > 0}
    {#each tradeData as trade (trade.id)}
      {trade.size}@{trade.price}
      <br />
    {/each}
  {:else}
    connecting...
  {/if}
</div>
