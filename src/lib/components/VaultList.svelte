<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { push } from "svelte-spa-router";
  import { vaultStore } from "../stores";
  import { getModalStore, type ModalSettings } from "@skeletonlabs/skeleton";

  const modalStore = getModalStore();
  let vaults: string[] = [];

  const listVaults = async () => {
    invoke("list_vaults")
      .then((response) => {
        console.log(response)
        vaults = response;
      })
      .catch((error) => {
        console.log("Error:", error);
      });
  };

  const selectVault = (name: string) => {
    const selectVaultModal: ModalSettings = {
      type: "prompt",
      title: "Password",
      body: "Provide password for the selected vault",
      value: "",
      valueAttr: { type: "password", required: true },
      response: async (masterPassword: string) => {
        if (masterPassword) {
          try {
            let masterPasswordId = await invoke("select_vault", {
              name: name?.toLowerCase(),
              masterPassword,
            });
            vaultStore.set({ name, masterPasswordId });
            push("/dashboard");
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
          invoke("add_vault", { name: name?.toLowerCase(), password })
            .then(() => {
              console.log("New vault successfully added");
            })
            .catch((error) => {
              console.log("Failed to create new vault");
              console.log("Error", error);
            });
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
