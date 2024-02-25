<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { appDataDir } from "@tauri-apps/api/path";
  import { appLocalDataDir } from "@tauri-apps/api/path";
  import TopBar from "../components/TopBar.svelte";
  import { databaseStore } from "../stores";
  import { push } from "svelte-spa-router";

  // TODO: implement windows acc & key file authentication
  let password: String;

  const handleSubmit = async () => {
    if ($databaseStore && password) {
      const result = await invoke("authorize", {
        database: $databaseStore.database,
        password,
      });
      console.log(result);

      push('/dashboard')
    }
  };
</script>

<TopBar searchBar={false} />
<div class="flex items-center justify-center mt-16">
  <form
    class="bg-slate-500 p-24 rounded-3xl"
    on:submit|preventDefault={handleSubmit}
  >
    <label for="password">Master password</label>
    <input class="input" type="password" name="password" bind:value={password} />
    <div class="mt-4 flex justify-end gap-4">
      <button type="submit">Ok</button>
      <button>Cancel</button>
    </div>
  </form>
</div>
