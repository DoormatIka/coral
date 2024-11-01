<script lang="ts">
  import Card from "$lib/Card.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import {onMount} from "svelte";

  const image_link = "https://cdn.discordapp.com/attachments/1063526734969974907/1301948621083775046/IMG_2547.jpg?ex=672655a4&is=67250424&hm=d6fbaccce10893532adc666ea92d6cab7a235353f49cd774f0656bb04b80929d&";

  type Character = {
    id: string,
    name: string,
    description: string,
    system_message: string,
    first_message: string,
    conversations: string[],
  }

  async function getCharacters() {
    try {
      characters = await invoke("grab_character_list");
    } catch (err) {
      console.log(err);
    }
  }
  
  let characters: Character[] = [];
  onMount(() => {
    getCharacters();
  })
</script>

<div class="px-5 pb-10 overflow-y-auto w-full h-full">
    <div class="grid xl:grid-cols-5 md:grid-cols-2 grid-cols-1 gap-5">
      {#each characters as char}
        <div class="m-auto h-48 my-7 w-full">
          <a href="/chat">
            <Card title={char.name} description={char.description} link={image_link}/>
          </a>
        </div>
      {/each}
    </div>
</div>

