<script lang="ts">
  if (!import.meta.env.SSR) {
    const apiUrl =
      import.meta.env.PROD === true
        ? "https://api.hodl.commonlab-van.com/api/btc"
        : "http://0.0.0.0:9000/api/btc";

    new EventSource(apiUrl).addEventListener(
      "message",
      (e) => (data = JSON.parse(e.data))
    );
  }

  let data: any;
  let ready = false;
  let latestPrice = 0;
  $: if (data) {
    ready = true;
    latestPrice =
      data && data.e === "aggTrade" && data.p ? data.p : latestPrice;
  }
</script>

<div class="row">
  <h1>btc price</h1>

  {#if ready}
    {latestPrice}
  {:else}
    connecting...
  {/if}
</div>
