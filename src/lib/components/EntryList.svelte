<script lang="ts">
  import { Avatar } from "@skeletonlabs/skeleton";
  import type { Entry } from "../models/Entry";

  export let entries: Entry[];
  export let selectedEntry: Entry | null = null;
  let filterTerm = ""

  type GroupedEntries = {
    [key: string]: Entry[];
  };

  entries.shift();
  const sortedEntries = entries.sort((a, b) => a.title.localeCompare(b.title));

  const groups = sortedEntries.reduce((acc: GroupedEntries, entry: Entry) => {
    const letter: string = entry.title[0].toUpperCase();
    if (!acc[letter]) {
      acc[letter] = [];
    }
    acc[letter].push(entry);
    return acc;
  }, {} as GroupedEntries);

  const selectEntry = (entry: Entry) => {
    selectedEntry = entry;
  };
</script>

<div
  class="w-64 flex flex-col h-screen bg-surface-800 drop-shadow-xl overflow-y-scroll"
>
  <div>
    <input class="input" type="text" placeholder="Filter" bind:value={filterTerm}/>
  </div>

  <ul class="flex-1">
    {#each Object.entries(groups) as [letter, entries]}
      <li class="text-xl p-1 pl-4">
        <strong>{letter.toUpperCase()}</strong>
      </li>
      {#each entries as entry}
        <li
          class="card-hover bg-surface-900 border border-x-2 border-surface-800 {entry ===
          selectedEntry
            ? 'active'
            : ''}"
        >
          <button
            class="w-full flex items-center m-2"
            on:click={() => selectEntry(entry)}
          >
            <Avatar
              width="w-12"
              rounded="rounded-2xl"
              background="bg-surface-600"
            />
            <p class="text-xl mx-4">{entry.title}</p>
          </button>
        </li>
      {/each}
    {/each}
  </ul>
</div>

<style>
  .active {
    background-color: #222;
  }
</style>
