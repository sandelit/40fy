<script lang="ts">
  import { Avatar } from "@skeletonlabs/skeleton";
  import type { Entry } from "../models/Entry";
  export let entries: Entry[];

  type GroupedEntries = {
    [key: string]: Entry[];
  };

  entries = [entries[0]];
  entries = [...entries, ...entries];
  entries.push({
    id: "12",
    title: "test",
    url: "test",
    username: "testtest",
    email: "testsson@",
    password: "123",
  });
  entries.push({
    id: "12",
    title: "rest",
    url: "test",
    username: "testtest",
    email: "testsson@",
    password: "123",
  });
  entries.push({
    id: "12",
    title: "lest",
    url: "test",
    username: "testtest",
    email: "testsson@",
    password: "123",
  });
  entries.push({
    id: "12",
    title: "est",
    url: "test",
    username: "testtest",
    email: "testsson@",
    password: "123",
  });
  entries.push({
    id: "12",
    title: "nest",
    url: "test",
    username: "testtest",
    email: "testsson@",
    password: "123",
  });

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
    console.log(entry);
  };
</script>

<div class="flex w-1/3 flex-col h-1/4 bg-surface-800 drop-shadow-xl">
  <div>filterbar</div>

  <ul>
    {#each Object.entries(groups) as [letter, entries]}
      <li class="text-xl p-1 pl-4">
        <strong>{letter.toUpperCase()}</strong>
      </li>
      {#each entries as entry}
        <li
          class="card-hover bg-surface-900 border border-x-2 border-surface-800"
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

  <!-- 

  <ul class="bg-primary-300">
    {#each listEntries as { letter, title }}
      {#if letter !== currentLetter}
        <li class="bg-primary-400 text-2xl">
          {(currentLetter = letter).toUpperCase()}
        </li>
      {/if}
      <div class="flex bg-primary-700">
        <Avatar
          initials={letter}
          rounded="rounded-3xl"
          background="bg-primary-800"
        />
        <li>{title}</li>
      </div>
    {/each}
  </ul>
-->
</div>
