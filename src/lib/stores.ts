import { localStorageStore } from "@skeletonlabs/skeleton";
import { writable } from "svelte/store";

export const vaultStore = writable({ name: "", masterPasswordId: "" });
