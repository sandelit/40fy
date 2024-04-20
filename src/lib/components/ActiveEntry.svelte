<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import CopyIcon from "./icons/CopyIcon.svelte";
  import type { Entry } from "../models/Entry";
  import { clipboard } from "@skeletonlabs/skeleton";

  export let entry: Entry | null = null;

  const styleClick = (event: any) => {
    event.target.parentElement.classList.add("scale-1");
    event.target.parentElement.classList.remove("hover:scale-105");
    setTimeout(() => {
      event.target.parentElement.classList.add("hover:scale-105");
      event.target.parentElement.classList.remove("scale-1");
    }, 100);
  };
</script>

<div class="flex-1 flex mt-16 justify-center">
  <div
    class="h-fit rounded-2xl bg-surface-300 dark:bg-surface-900 border border-opacity-80 border-surface-400 dark:border-surface-800 drop-shadow-xl p-12"
  >
    <div>
      <h1 class="text-center mb-12 text-2xl">
        {entry?.title || "_"}
      </h1>
    </div>

    <div>
      <button
        class="w-full border-b border-opacity-30 border-surface-400 p-4"
        use:clipboard={entry?.email || ""}
        on:click={styleClick}
      >
        <div class="flex justify-between hover:scale-105">
          <pre>Email:     {entry?.email || ""}</pre>
          <CopyIcon />
        </div>
      </button>
    </div>

    <div>
      <button
        class="w-full border-b border-opacity-30 border-surface-400 p-4"
        use:clipboard={entry?.username || ""}
        on:click={styleClick}
      >
        <div class="flex justify-between hover:scale-105">
          <pre>Username:  {entry?.username || ""}</pre>
          <CopyIcon />
        </div>
      </button>
    </div>

    <div>
      <button
        class="w-full p-4"
        use:clipboard={entry?.password || ""}
        on:click={styleClick}
      >
        <div class="flex justify-between hover:scale-105">
          <pre>Password:  {entry ? "********" : ""}</pre>
          <CopyIcon />
        </div>
      </button>
    </div>

    {#if entry?.updated_at_date_time && entry?.created_at_date_time}
      <div class="opacity-50 grid grid-cols-2 gap-x-4 mt-12">
        <p class="text-right">Last modified:</p>
        <p>{entry?.updated_at_date_time}</p>
        <p class="text-right">Created:</p>
        <p>{entry?.created_at_date_time}</p>
      </div>
    {/if}
  </div>
</div>
