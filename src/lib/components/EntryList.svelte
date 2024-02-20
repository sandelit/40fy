<script lang="ts">
  import { Avatar } from "@skeletonlabs/skeleton";
  import type { Entry } from "../models/Entry";
  export let entries: Entry[];

  type GroupedEntries = {
    [key: string]: string[];
  };

  entries = [entries[0]];
  entries = [...entries, ...entries];
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
    acc[letter].push(entry.title);
    return acc;
  }, {} as GroupedEntries);
</script>

<div class="flex flex-1 flex-col bg-shark-950 h-1/4">
  <div>filterbar</div>

  <ul class="bg-primary-300">
    {#each Object.entries(groups) as [letter, entries]}
      <li class="bg-primary-400 text-2xl">
        <strong>{letter.toUpperCase()}</strong>
      </li>
      {#each entries as entry}
        <li>
          <div class="flex bg-primary-700">
            <Avatar
              initials={letter}
              rounded="rounded-3xl"
              background="bg-primary-800"
            />
            <p class="text-xl">{entry}</p>
          </div>
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
