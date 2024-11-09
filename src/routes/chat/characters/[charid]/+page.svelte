
<script lang="ts">
  import type { PageData } from './$types';
  import type { CharacterConversation } from "../../../types";
  import {invoke} from '@tauri-apps/api/core';
  import {page} from '$app/stores';
  import { base } from '$app/paths'

  const charid = $page.params["charid"];

  let { data }: { data: PageData } = $props();

  const name = data.character.name;
  const description = data.character.description;

  let conversations: CharacterConversation[] = $state(data.conversations);

  let error: string = $state("");
  function addConversation() {
    invoke("add_conversation", { "charId": charid })
      .catch((err) => error = err);
    invoke("grab_conversation_list", {"charId": charid})
      .then((c) => {
        const r = c as CharacterConversation[];
        conversations = r;
      })
      .catch((err) => error = err);
  }
</script>

<br class="py-10 h-full">
<div class="w-full h-full p-5 overflow-y-auto">

  <div class="flex flex-col justify-center items-center">
    <div class="flex justify-center items-center">
      <div class="avatar">
        <div class="sm:w-20 w-14 rounded-full">
          <img src="{base}/no_image.png" alt="avatar" />
        </div>
      </div>
      <h2 class="sm:text-4xl pl-5 text-2xl">{name}</h2>
    </div>
    <br>
    <div class="w-full max-w-3xl px-14 text-wrap break-all truncate">
      <h3>Description:</h3>
      <p>{description.slice(0, 500)}{description.length > 500 ? "..." : ""}</p>
    </div>
    <div class="w-full max-w-3xl">
      <div class="divider"></div>
    </div>
  </div>


  <br>

  <div class="w-full h-full flex flex-col items-center">

    <div class="w-full max-w-3xl flex flex-col justify-center join join-vertical gap-2 pb-10">
      {#each conversations as conversation, index (conversation["id"])}
        <a data-sveltekit-preload-data="off" href="/chat/{conversation.id}" class="join-item border border-sm border-neutral hover:bg-neutral transition-colors p-5 max-h-20 truncate text-ellipsis">
          C{index + 1}: {conversation.log.at(-1)!.content}
        </a>
      {/each}
      <button class="btn btn-outline btn-default join-item" onclick={() => addConversation()} aria-label="addConversation">
        <iconify-icon class="text-3xl" icon="mdi:plus"></iconify-icon>
      </button>
    </div>

    <br>
    {#if error.length > 0}
      <p class="text-error">Error: {error}</p>
    {/if}

  </div>

</div>
