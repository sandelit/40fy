<script>
  import VaultList from "../components/VaultList.svelte";
  import TopBar from "../components/TopBar.svelte";
  import { link } from "svelte-spa-router";
</script>

<TopBar searchBar={false} />
<VaultList />
