<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import CopyIcon from "./icons/CopyIcon.svelte";
  import type { Entry } from "../models/Entry";

  export let entry: Entry | null = null;

  const copyToClipboard = async (event) => {
    event.target.parentElement.classList.add("scale-1");
    event.target.parentElement.classList.remove("hover:scale-105");
    setTimeout(() => {
      event.target.parentElement.classList.add("hover:scale-105");
      event.target.parentElement.classList.remove("scale-1");
    }, 100);
    invoke("copy_to_clipboard", { text: "test" });
  };
</script>

<div class="h-screen w-1/3">
  <div
    class="h-1/2 rounded-2xl bg-surface-900 mt-16 mr-32 border border-opacity-80 border-surface-800 drop-shadow-xl p-12"
  >
    {#if entry}
      <!-- content here -->
      <div><h1 class="text-center mb-12 text-2xl">{entry.title}</h1></div>

      <div>
        <button
          class="w-full border-b border-opacity-30 border-surface-400 p-4"
          on:click={copyToClipboard}
        >
          <div class="flex justify-between hover:scale-105">
            <pre>Email:     {entry.email}</pre>
            <CopyIcon />
          </div>
        </button>
      </div>

      <div>
        <button
          class="w-full border-b border-opacity-30 border-surface-400 p-4"
          on:click={copyToClipboard}
        >
          <div class="flex justify-between hover:scale-105">
            <pre>Username:  {entry.username}</pre>
            <CopyIcon />
          </div>
        </button>
      </div>

      <div>
        <button class="w-full p-4" on:click={copyToClipboard}>
          <div class="flex justify-between hover:scale-105">
            <pre>Password:  ********</pre>
            <CopyIcon />
          </div>
        </button>
      </div>
    {/if}
  </div>
</div>
