<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { push } from "svelte-spa-router";
  import { vaultStore } from "../stores";
  import { getModalStore, type ModalSettings } from "@skeletonlabs/skeleton";

  const modalStore = getModalStore();
  let vaults: string[] = [];

  const listVaults = async () => {
    vaults = await invoke("list_vaults");
  };

  const selectVault = (name: string) => {
    const selectVaultModal: ModalSettings = {
      type: "prompt",
      title: "Password",
      body: "Provide password for the selected vault",
      value: "",
      valueAttr: { type: "text", required: true },
      response: async (masterPassword: string) => {
        if (masterPassword) {
          try {
            const vault = await invoke("select_vault", {
              name: name?.toLowerCase(),
              masterPassword,
            });

            console.log(name);
            vaultStore.set(name);

            push("/dashboard");

            console.log(vault);
          } catch (error) {
            console.log(error);
          }
        }
      },
    };
    modalStore.trigger(selectVaultModal);
  };

  const addVault = () => {
    const addVaultModal: ModalSettings = {
      type: "component",
      title: "New Vault",
      component: "addVaultModal",
      response: ({ name, password }) => {
        if (name && password) {
          if (vaults.includes(name)) {
            console.log("Vault already exists");
            return;
          }
          invoke("add_vault", { name: name?.toLowerCase(), password });
          listVaults();
        }
      },
    };
    modalStore.trigger(addVaultModal);
  };

  onMount(listVaults);
</script>

<div class="flex justify-center adjust mt-8">
  <div class="p-6 rounded-3xl bg-surface-900">
    <button class="btn variant-filled mb-4" on:click={addVault}
      >Add New Vault</button
    >
    <ul class="bg-surface-800">
      {#each vaults as vault}
        <li class="odd:bg-surface-700 p-2 card card-hover">
          <button on:click={() => selectVault(vault)}>
            {vault.split("/").slice(-1)}
          </button>
        </li>
      {/each}
    </ul>
  </div>
</div>
