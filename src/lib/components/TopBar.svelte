<script>
  import { LightSwitch } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api/core";
  import { databaseStore } from "../stores";

  export let searchBar = true;

  /////////////////////////////
  const addEntry = () => {
    let database = $databaseStore.database;

    invoke("add_password", {
      id: "1",
      title: "testtitle",
      url: "www.com",
      username: "userrrrr",
      email: "user@user.com",
      password: "123en",
      database,
    });
  };
  /////////////////////////////
</script>

<div
  class="bg-surface-900 border-b border-opacity-30 border-surface-400 flex justify-between pr-4 py-4"
>
  <div />
  {#if searchBar}
    <div class="flex pb-2 gap-2">
      <input
        type="text"
        name="search"
        id="search"
        placeholder="search"
        class="input"
      />
      <button class="btn variant-soft" on:click={addEntry}>+</button>
    </div>
  {/if}
  <LightSwitch rounded={"rounded-lg"} width={"w-16"} height={"h-8"} />
</div>
