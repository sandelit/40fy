import { localStorageStore } from "@skeletonlabs/skeleton";
import type { Writable } from "svelte/store";

export const databaseStore: Writable<string> = localStorageStore(
  "database",
  "",
);
