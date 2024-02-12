<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  const copyToClipboard = async () => {
    invoke("copy_to_clipboard", { text: "test" });
    try {
      invoke("create_user", { name: "test", age: 32 });
      console.log("user created");
    } catch (error) {
      console.log("user error creation");
    }

    try {
      const users = await invoke("read_users");
      console.log("Users:", users);
      // Here you would typically update the DOM with the fetched user data
    } catch (e) {
      console.error("Error fetching users:", e);
    }
  };
</script>

<div class="flex-1 flex items-center justify-center">
  <div class="bg-shark-950 rounded-2xl">
    <div><h1 class="text-center mb-12 text-2xl">Reddit</h1></div>

    <div
      class="border-b border-opacity-30 border-shark-50 p-4 cursor-pointer flex justify-between"
    >
      <div>Email:</div>
      <div>testman@test.com</div>
      <div>icon</div>
    </div>
    <div
      class="border-b border-opacity-30 border-shark-50 p-4 cursor-pointer flex justify-between"
    >
      <div>Username:</div>
      <div>Testman</div>
      <div>icon</div>
    </div>
    <div
      class="p-4 cursor-pointer flex justify-between"
      on:click={copyToClipboard}
    >
      <div>Password:</div>
      <div>*********</div>
      <div>icon</div>
    </div>
  </div>
</div>
