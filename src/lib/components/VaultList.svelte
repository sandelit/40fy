<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { push } from "svelte-spa-router";
  import { vaultStore } from "../stores";
  import {
    getModalStore,
    type ModalSettings,
  } from "@skeletonlabs/skeleton";

  const modalStore = getModalStore();
  let vaults: string[] = [];

  const listVaults = async () => {
    vaults = await invoke("list_vaults");
    console.log(vaults);
  };

  const selectVault = (vault: string) => {
    vaultStore.set({ vault });
    push("/login?data=" + vault);
  };

  const addVault = () => {
    const modal: ModalSettings = {
      type: "component",
      title: "New Vault",
      component: "addVaultModal",
      response: ({ name, password }) =>
        invoke("add_vault", { name: name?.toLowerCase(), password }),
    };
    modalStore.trigger(modal);
  };

  onMount(listVaults);
</script>

<div class="flex justify-center adjust mt-8">
  <div class="p-6 rounded-3xl bg-surface-900">
    <button class="btn variant-filled mb-4" on:click={addVault}
      >Add New Vault</button
    >
    <ul class="bg-surface-800">
      {#each vaults as vault}
        <li class="odd:bg-surface-700 p-2 card card-hover">
          <button on:click={() => selectVault(vault)}>
            {vault.split("/").slice(-1)}
          </button>
        </li>
        <li class="odd:bg-surface-700 p-2 card card-hover">
          <button on:click={() => selectVault(vault)}>
            {vault.split("/").slice(-1)}
          </button>
        </li>
        <li class="odd:bg-surface-700 p-2 card card-hover">
          <button on:click={() => selectVault(vault)}>
            {vault.split("/").slice(-1)}
          </button>
        </li>
      {/each}
    </ul>
  </div>
</div>
