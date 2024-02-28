<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { push } from "svelte-spa-router";
  import { vaultStore } from "../stores";

  let vaults: string[] = [];

  const listVaults = async () => {
    vaults = await invoke("list_vaults");
    console.log(vaults)
  };

  const selectVault = (vault: string) => {
    vaultStore.set({ vault });
    push("/login?data=" + vault);
  };

  onMount(listVaults);
</script>

<div class="flex justify-center adjust mt-8">
  <div class="w-1/2 rounded-xl">
    <ul class="bg-slate-900">
      {#each vaults as vault}
        <li class="odd:bg-slate-800 p-2 card card-hover">
          <!-- on:click={() => selectDatabase(database)} -->
          <a on:click={() => selectVault(vault)}>
            {vault.split("/").slice(-1)}
          </a>
        </li>
      {/each}
    </ul>
  </div>
</div>
