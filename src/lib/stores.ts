import { localStorageStore } from "@skeletonlabs/skeleton";
import { writable, type Writable } from "svelte/store";

export const vaultStore: Writable<string> = writable("")
