import preprocess from "svelte-preprocess";
import adapter from "@sveltejs/adapter-static";

const isGHPages = process.env.GITHUB_PAGES == "true";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: preprocess(),

  kit: isGHPages
    ? {
        adapter: adapter(),

        // hydrate the <div id="svelte"> element in src/app.html
        target: "#svelte",

        appDir: "_app",
        paths: {
          base: "/hodl",
        },
      }
    : {
        adapter: adapter(),

        // hydrate the <div id="svelte"> element in src/app.html
        target: "#svelte",
      },
};

export default config;
