<script lang="ts">
  import Card from "$lib/Card.svelte";
  // import { invoke } from "@tauri-apps/api/core";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // greetMsg = await invoke("greet", { name });
  }
  const characters = [
    {
      "title": "Satori Komeiji",
      "description": "The Mistress of the Palace of the Earth Spirits.",
      "image": "https://safebooru.org/thumbnails/2/thumbnail_0fd6aaebb040008753fd92380448e68c7267e9b8.jpg?5295036"
    },
    {
      "title": "Usami Sumireko",
      "description": "Usami Sumireko, the outside.",
      "image": "https://safebooru.org/thumbnails/514/thumbnail_40a551e80c803b4210d1f42b41885653123a70fe.jpg?5302041"
    },
    {
      "title": "Fujiwara no Mokou",
      "description": "Fujiwara no Mokou formerly an ordinary human, but she became an immortal being after drinking the Hourai Elixir about 1300 years ago.",
      "image": "https://en.touhouwiki.net/images/thumb/0/04/Th155Mokou.png/488px-Th155Mokou.png"
    },
  ];
</script>

<div class="px-5 pb-10 overflow-y-auto w-full h-full">
    <div class="grid xl:grid-cols-5 md:grid-cols-2 grid-cols-1 gap-5">
      {#each characters as char}
        <div class="m-auto h-48 my-7 w-full">
          <a href="/chat">
            <Card title={char.title} description={char.description} link={char.image}/>
          </a>
        </div>
      {/each}
    </div>
</div>

