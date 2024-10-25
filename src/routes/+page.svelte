<script lang="ts">
  import Card from "$lib/Card.svelte";
  // import { invoke } from "@tauri-apps/api/core";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // greetMsg = await invoke("greet", { name });
  }
  const characters = [
    { "title": "Satori Komeiji Satori Komeiji Satori Komeiji Satori Komeiji ", "description": "The Mistress of the Palace of the Earth Spirits after the underground city was separated from Hell. Her ability to read minds causes various youkai and spirits to fear her, but makes her loved by the animals that normally can't be understood. With these animals as pets, she manages the ruins of the Hell of Blazing Fires where her home stands. She is also the main protagonist of Foul Detective Satori. ", "image": "https://safebooru.org/thumbnails/2/thumbnail_0fd6aaebb040008753fd92380448e68c7267e9b8.jpg?5295036" },
    { "title": "Sa", "description": "lol", "image": "nan" }
  ];
</script>

<div class="p-5 overflow-y-auto w-full h-full">
    <div class="grid xl:grid-cols-5 md:grid-cols-3 grid-cols-1 gap-5">
      {#each characters as char}
        <div class="m-auto h-48 my-7 w-full">
          <a href="/chat">
            <Card title={char.title} description={char.description} link={char.image}/>
          </a>
        </div>
      {/each}
    </div>
</div>

