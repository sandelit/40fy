<script lang="ts">
  import Dashboard from "./lib/pages/Dashboard.svelte";
  import LoginPage from "./lib/pages/LoginPage.svelte";
  import HomePage from "./lib/pages/HomePage.svelte";
  import CreateDbPage from "./lib/pages/CreateDbPage.svelte";
  import TopBar from "./lib/components/TopBar.svelte";
  import Router from "svelte-spa-router";

  let db = true;
  let auth = false;

  const routes = {
    "/": HomePage,
    "/login": LoginPage,
    "/dashboard": Dashboard,
  };
</script>

<main class="h-screen overflow-y-hidden">
  <Router {routes} />
</main>
<!-- 
<main class="h-screen">
  {#if db}
    {#if auth}
      <Dashboard />
    {:else}
      <TopBar searchBar={false} />
      <div class="flex items-center justify-center h-full">
        <MasterPassForm />
      </div>
    {/if}
  {:else}
    <CreateDbPage />
  {/if}
</main>
<main class="h-screen">
  {#if !auth}
    <TopBar searchBar={false} />
    <div class="flex items-center justify-center h-screen">
      <MasterPassForm />
    </div>
  {:else}
    <TopBar />
    <div class="flex">
      <SideBar />
      <EntryList />
      <ActiveEntry />
    </div>
  {/if}
</main>
-->
