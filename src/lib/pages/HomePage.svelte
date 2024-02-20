<script>
  import DatabaseList from "../components/DatabaseList.svelte";
  import TopBar from "../components/TopBar.svelte";
  import { link } from "svelte-spa-router";
</script>

<TopBar searchBar={false} />
<DatabaseList />
