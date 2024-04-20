<script lang="ts">
  import Dashboard from "./lib/pages/Dashboard.svelte";
  import HomePage from "./lib/pages/HomePage.svelte";
  import Router from "svelte-spa-router";
  import {
    Modal,
    initializeStores,
    type ModalComponent,
  } from "@skeletonlabs/skeleton";
  import AddVaultModal from "./lib/components/modals/AddVaultModal.svelte";
  import AddEntryModal from "./lib/components/modals/AddEntryModal.svelte";
  import { onMount } from "svelte";
  import { autoModeWatcher } from "@skeletonlabs/skeleton";

  onMount(() => {
    autoModeWatcher();
  });

  const modalRegistry: Record<string, ModalComponent> = {
    addVaultModal: { ref: AddVaultModal },
    addEntryModal: { ref: AddEntryModal },
  };

  initializeStores();

  const routes = {
    "/": HomePage,
    "/dashboard": Dashboard,
  };
</script>

<main class="h-screen overflow-y-hidden text-neutral-900 dark:text-neutral-200" >
  <Modal components={modalRegistry} />
  <Router {routes} />
</main>
