<script lang="ts">
  import Dashboard from "./lib/pages/Dashboard.svelte";
  import LoginPage from "./lib/pages/LoginPage.svelte";
  import HomePage from "./lib/pages/HomePage.svelte";
  import Router from "svelte-spa-router";
  import {
    Modal,
    initializeStores,
    type ModalComponent,
  } from "@skeletonlabs/skeleton";
  import AddVaultModal from "./lib/components/modals/AddVaultModal.svelte";
    import AddEntryModal from "./lib/components/modals/AddEntryModal.svelte";

  const modalRegistry: Record<string, ModalComponent> = {
    // Set a unique modal ID, then pass the component reference
    addVaultModal: { ref: AddVaultModal },
    addEntryModal: { ref: AddEntryModal },
  };

  initializeStores();

  let db = true;
  let auth = false;

  const routes = {
    "/": HomePage,
    "/login": LoginPage,
    "/dashboard": Dashboard,
  };
</script>

<main class="h-screen overflow-y-hidden">
  <Modal components={modalRegistry} />
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
