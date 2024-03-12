<script lang="ts">
  import {
    LightSwitch,
    getModalStore,
    type ModalSettings,
  } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api/core";
  import { vaultStore } from "../stores";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
  const modalStore = getModalStore();

  export let searchBar = true;

  const addEntry = () => {
    let vault = $vaultStore.name;
    let masterPasswordId = $vaultStore.masterPasswordId;

    const addEntryModal: ModalSettings = {
      type: "component",
      component: "addEntryModal",
      response: ({ title, url, username, email, password }) => {
        try {
          invoke("add_vault_entry", {
            title,
            url,
            username,
            email,
            password,
            masterPasswordId,
            vault,
          });
          dispatch("entryAdded");
        } catch (error) {
          console.log("Failed to create new entry");
          console.log("Error:", error);
        }
      },
    };
    modalStore.trigger(addEntryModal);
  };
</script>

<div
  class="bg-surface-900 border-b border-opacity-30 border-surface-400 flex justify-between pr-4 py-4"
>
  <div />
  {#if searchBar}
    <div class="flex gap-2">
      <input
        type="text"
        name="search"
        id="search"
        placeholder="search"
        class="input p-2"
      />
      <button class="btn variant-soft" on:click={addEntry}>+</button>
    </div>
  {/if}
  <LightSwitch rounded={"rounded-lg"} width={"w-16"} height={"h-8"} />
</div>
