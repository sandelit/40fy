<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ActiveEntry from "../components/ActiveEntry.svelte";
  import EntryList from "../components/EntryList.svelte";
  import SideBar from "../components/SideBar.svelte";
  import TopBar from "../components/TopBar.svelte";
  import { databaseStore } from "../stores";

  const readPassword = async (database: string) => {
    try {
      const entries = await invoke("read_passwords", { database });
      console.log("entries:", entries);
      return entries;
    } catch (e) {
      console.error("Error fetching entries:", e);
    }
  };
</script>

{#await readPassword($databaseStore.database) then entries}
  <TopBar />
  <div class="flex justify-between gap-16">
    <SideBar />
    <EntryList {entries} />
    <ActiveEntry />
  </div>
{/await}
