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

<div class="flex justify-between pr-4 pt-4">
  <div />
  {#if searchBar}
    <div>
      <input
        type="text"
        name="search"
        id="search"
        placeholder="search"
        class="border border-black"
      />
      <button on:click={addEntry}>+</button>
    </div>
  {/if}
  <LightSwitch rounded={"rounded-lg"} width={"w-16"} height={"h-8"} />
</div>
