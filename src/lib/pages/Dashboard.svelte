<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ActiveEntry from "../components/ActiveEntry.svelte";
  import EntryList from "../components/EntryList.svelte";
  import SideBar from "../components/SideBar.svelte";
  import TopBar from "../components/TopBar.svelte";
  import { vaultStore } from "../stores";
  import type { Entry } from "../models/Entry";

  let selectedEntry: Entry | null = null;

  const readPassword = async (): Promise<Entry[]> => {
    return invoke("read_passwords", { vault: $vaultStore.vault });
  };

  let entriesPromise = readPassword();
</script>

<TopBar
  on:entryAdded={() => {
    entriesPromise = readPassword();
  }}
/>
<div class="flex">
  <SideBar />
  {#await entriesPromise then entries}
    <EntryList {entries} bind:selectedEntry />
  {/await}
  <ActiveEntry entry={selectedEntry} />
</div>
