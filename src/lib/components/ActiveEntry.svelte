<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import CopyIcon from "./icons/CopyIcon.svelte";

  const copyToClipboard = async () => {
    invoke("copy_to_clipboard", { text: "test" });
  };
</script>

<div class="flex mr-32">
  <div
    class="h-1/2 rounded-2xl bg-surface-900 mt-16 border border-opacity-80 border-surface-800 drop-shadow-xl p-12"
  >
    <div><h1 class="text-center mb-12 text-2xl">Reddit</h1></div>

    <div>
      <button
        class="border-b border-opacity-30 border-surface-400 p-4 cursor-pointer flex justify-between"
        on:click={copyToClipboard}
      >
        <div>Email:</div>
        <div>testman@test.com</div>
        <CopyIcon />
      </button>
    </div>

    <div>
      <button
        class="border-b border-opacity-30 border-surface-400 p-4 cursor-pointer flex justify-between"
        on:click={copyToClipboard}
      >
        <div>Username:</div>
        <div>Testman</div>
        <CopyIcon />
      </button>
    </div>

    <div>
      <button
        class="p-4 cursor-pointer flex justify-between"
        on:click={copyToClipboard}
      >
        <div>Password:</div>
        <div>*********</div>
        <CopyIcon />
      </button>
    </div>
  </div>
</div>
