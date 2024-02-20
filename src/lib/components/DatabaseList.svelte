<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { appDataDir } from "@tauri-apps/api/path";
  import { onMount } from "svelte";
  import { push } from "svelte-spa-router";
  import { databaseStore } from "../stores";

  let databases: string[] = [];

  const fetchDatabases = async () => {
    const dirpath = await appDataDir();
    databases = await invoke("list_databases", { dirpath });
  };

  const selectDatabase = (database: string) => {
    databaseStore.set({ database });
    push("/login?data=" + database);
  };

  onMount(fetchDatabases);
</script>

<div class="flex justify-center adjust mt-8">
  <div class="w-1/2 rounded-xl">
    <ul class="bg-slate-900">
      {#each databases as database}
        <li class="odd:bg-slate-800 p-2 card card-hover">
          <!-- on:click={() => selectDatabase(database)} -->
          <a on:click={() => selectDatabase(database)}>
            {database.split("/").slice(-1)}
          </a>
        </li>
      {/each}
    </ul>
  </div>
</div>
