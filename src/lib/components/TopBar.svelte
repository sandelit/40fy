<script>
  import { LightSwitch } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api/core";
  import { vaultStore } from "../stores";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let searchBar = true;

  /////////////////////////////
  const addEntry = () => {
    let vault = $vaultStore.vault;

    invoke("add_password", {
      id: "12341112",
      title: "tttttttt",
      url: "www.com",
      username: "userrrrr",
      email: "user@user.com",
      password: "123en",
      vault,
    });
    dispatch("entryAdded");
  };
  /////////////////////////////
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
