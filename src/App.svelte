<script lang="ts">
  import ActiveEntry from "./lib/components/ActiveEntry.svelte";
  import EntryList from "./lib/components/EntryList.svelte";
  import SideBar from "./lib/components/SideBar.svelte";
  import TopBar from "./lib/components/TopBar.svelte";
</script>

<main class="h-screen">
  <TopBar />
  <div class="flex">
    <SideBar />
    <EntryList />
    <ActiveEntry />
  </div>
</main>
