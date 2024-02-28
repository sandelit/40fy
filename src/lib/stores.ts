import { localStorageStore } from "@skeletonlabs/skeleton";
import type { Writable } from "svelte/store";

export const vaultStore: Writable<string> = localStorageStore(
  "vault",
  "",
);
