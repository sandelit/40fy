<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import CopyIcon from "./icons/CopyIcon.svelte";

  const copyToClipboard = async () => {
    invoke("copy_to_clipboard", { text: "test" });
  };
</script>

<div class="card flex-1 flex items-center justify-center">
  <div class="rounded-2xl">
    <div><h1 class="text-center mb-12 text-2xl">Reddit</h1></div>

    <button
      class="border-b border-opacity-30 border-shark-50 p-4 cursor-pointer flex justify-between"
    >
      <div>Email:</div>
      <div>testman@test.com</div>
      <CopyIcon />
    </button>
    <button
      class="border-b border-opacity-30 border-shark-50 p-4 cursor-pointer flex justify-between"
    >
      <div>Username:</div>
      <div>Testman</div>
      <CopyIcon />
    </button>

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
