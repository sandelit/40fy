<script lang="ts">
  import type { SvelteComponent } from "svelte";
  import { getModalStore } from "@skeletonlabs/skeleton";

  export let parent: SvelteComponent;

  const modalStore = getModalStore();
  const formData = {
    name: "",
    password: "",
  };

  function onFormSubmit(): void {
    if ($modalStore[0].response) $modalStore[0].response(formData);
    modalStore.close();
  }
</script>

{#if $modalStore[0]}
  <div class="modal-example-form card p-4 w-96 shadow-xl space-y-4">
    <header class="text-2xl font-bold">
      {$modalStore[0].title ?? "(title missing)"}
    </header>
    <article>{$modalStore[0].body ?? ""}</article>
    <!-- Enable for debugging: -->
    <form class="modal-form p-4 space-y-4">
      <label class="label">
        <span>Vault Name</span>
        <input class="input" type="text" bind:value={formData.name} />
      </label>
      <label class="label">
        <span>Password</span>
        <input class="input" type="password" bind:value={formData.password} />
      </label>
    </form>
    <footer class="modal-footer flex justify-end gap-2">
      <button class="btn variant-ghost" on:click={parent.onClose}>Cancel</button
      >
      <button class="btn variant-filled" on:click={onFormSubmit}
        >Submit Form</button
      >
    </footer>
  </div>
{/if}
