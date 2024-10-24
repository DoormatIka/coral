<script lang="ts">
  import Avatar from "$lib/Avatar.svelte";
    import Card from "$lib/Card.svelte";
  // import { invoke } from "@tauri-apps/api/core";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // greetMsg = await invoke("greet", { name });
  }
  const avatars = [
    "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp",
    "https://cdn.discordapp.com/attachments/914427100935647245/1298626589063381073/image.png?ex=671ae882&is=67199702&hm=09d4844da579a73b73180d3fa1b3f8b2fa93cbcf32c57f0ce17202e2ae21bcc9&",
    "https://cdn.discordapp.com/attachments/914427100935647245/1298618003021365309/FB_IMG_1729685032217.jpg?ex=671ae083&is=67198f03&hm=6587407ef9e9f3e65999552e0130877fa3e3b9e441941c3f73666066ce3276be&",
  ];
  const characters = [
    { "title": "Satori", "description": "Mistress of the Palace. aaaaaaaaaaaaaaaaaaaaaaaaa  bbbbbbbbbbbbbbbb cccccccccccccc dddddddddddddd", "image": "https://safebooru.org/thumbnails/2/thumbnail_0fd6aaebb040008753fd92380448e68c7267e9b8.jpg?5295036" },
    { "title": "Satori", "description": "Mistress of the Palace.", "image": "https://safebooru.org/thumbnails/2/thumbnail_0fd6aaebb040008753fd92380448e68c7267e9b8.jpg?5295036" },
    { "title": "Satori", "description": "Mistress of the Palace.", "image": "https://safebooru.org/thumbnails/2/thumbnail_0fd6aaebb040008753fd92380448e68c7267e9b8.jpg?5295036" },
    { "title": "Koishi", "description": "Satori's sister.", "image": "https://safebooru.org/thumbnails/4609/thumbnail_aa72813f4fac0b5e1ba739c4343c63ce7e32b8cb.jpg?5289926" },
    { "title": "Koishi", "description": "Satori's sister.", "image": "https://safebooru.org/thumbnails/4609/thumbnail_aa72813f4fac0b5e1ba739c4343c63ce7e32b8cb.jpg?5289926" },
    { "title": "Koishi", "description": "Satori's sister.", "image": "https://safebooru.org/thumbnails/4609/thumbnail_aa72813f4fac0b5e1ba739c4343c63ce7e32b8cb.jpg?5289926" },
    { "title": "Satori", "description": "Mistress of the Palace.", "image": "https://safebooru.org/thumbnails/2/thumbnail_0fd6aaebb040008753fd92380448e68c7267e9b8.jpg?5295036" },
    { "title": "Satori", "description": "Mistress of the Palace.", "image": "https://safebooru.org/thumbnails/2/thumbnail_0fd6aaebb040008753fd92380448e68c7267e9b8.jpg?5295036" },
    { "title": "Koishi", "description": "Satori's sister.", "image": "https://safebooru.org/thumbnails/4609/thumbnail_aa72813f4fac0b5e1ba739c4343c63ce7e32b8cb.jpg?5289926" },
    { "title": "Satori", "description": "Mistress of the Palace.", "image": "https://safebooru.org/thumbnails/2/thumbnail_0fd6aaebb040008753fd92380448e68c7267e9b8.jpg?5295036" },
    { "title": "Koishi", "description": "Satori's sister.", "image": "https://safebooru.org/thumbnails/4609/thumbnail_aa72813f4fac0b5e1ba739c4343c63ce7e32b8cb.jpg?5289926" },
    { "title": "Koishi", "description": "Satori's sister.", "image": "https://safebooru.org/thumbnails/4609/thumbnail_aa72813f4fac0b5e1ba739c4343c63ce7e32b8cb.jpg?5289926" },
  ];
</script>

<main>
  <div class="h-full w-full flex">

    <div class="min-w-max border-r border-neutral">
      <div class="p-5">
        <h1 class="mb-5">Coral</h1>
        <h2 class="my-2">History</h2>
        <div class="grid grid-cols-3 gap-2">
          {#each avatars as avatar}
            <div class="w-10"><Avatar avatar={avatar}/></div>
          {/each}
        </div>
      </div>
    </div>

    <div class="p-5 overflow-y-auto w-full">
        <div class="grid xl:grid-cols-5 md:grid-cols-3 sm:grid-cols-2 grid-cols-1 gap-2">
          {#each characters as char}
            <div class="m-auto h-48 my-7 w-full">
              <Card title={char.title} description={char.description} link={char.image}/>
            </div>
          {/each}
        </div>
    </div>

  </div>
</main>

