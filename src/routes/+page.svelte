<script lang="ts">
  import Card from "$lib/Card.svelte";
  import {invoke} from "@tauri-apps/api/core";
  import {onMount} from "svelte";
  import {type Character} from "./types";
  import { base } from '$app/paths'

  const image_link = `${base}/no_image.png`;
  onMount(async () => {
    await getCharacters();
  })

  let characters: Character[] = $state([]);
  async function getCharacters() {
    try {
      characters = await invoke("grab_character_list");
    } catch (err) {
      console.log(err);
    }
  }
</script>

<div class="px-5 py-10 overflow-y-auto w-full h-full">
    {#if characters.length > 0}
      <div class="sm:grid xl:grid-cols-4 sm:grid-cols-2 flex flex-col gap-5">
        {#each characters as char}
          <div class="m-auto w-full h-40 sm:h-auto">
            <Card 
              title={char.name} 
              description={char.description} 
              image_link={image_link} 
              link="/chat/{char.id}"
              removeCharacter={async () => {
                await invoke("delete_character", {id: char.id}) 
                await getCharacters();
              }}
            />
          </div>
        {/each}
      </div>
    {:else}
      <div class="flex flex-col justify-center items-center h-full gap-3">
        <h2 class="text-secondary-content opacity-60">No character made.</h2>
      </div>
    {/if}
</div>

