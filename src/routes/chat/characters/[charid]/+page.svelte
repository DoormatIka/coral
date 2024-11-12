
<script lang="ts">
  import type { PageData } from './$types';
  import type { CharacterConversation } from "../../../types";
  import {invoke} from '@tauri-apps/api/core';
  import {page} from '$app/stores';

  const charid = $page.params["charid"];

  let { data }: { data: PageData } = $props();

  const name = data.character.name;
  const description = data.character.description;

  let conversations: CharacterConversation[] = $state(data.conversations);

  let error: string = $state("");
  function addConversation() {
    invoke("add_conversation", { "charId": charid })
      .then(() => {
        invoke<CharacterConversation[]>("grab_conversation_list", {"charId": charid})
          .then((c) => { conversations = c; })
          .catch((err) => error = err);
      })
      .catch((err) => error = err);
  }

  function removeConversation(id: string) {
    invoke("delete_conversation", { "id": id })
      .then(() => {
        invoke<CharacterConversation[]>("grab_conversation_list", {"charId": charid})
          .then((c) => { conversations = c; })
          .catch((err) => error = err);
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
          <img src={data.character.image} alt="avatar" />
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
        <div class="join-item flex items-center border border-sm border-neutral hover:bg-neutral transition-colors px-5 max-h-20">

          <a data-sveltekit-preload-data="off" href="/chat/{conversation.id}" class="py-5 flex-1 truncate text-ellipsis">
            #{index + 1}: {conversation.log.at(-1)!.content}
          </a>

          <button class="btn btn-square btn-ghost btn-sm" style="vertical-align: -0.125em" onclick={() => {removeConversation(conversation.id)}} aria-label="close">
            <iconify-icon icon="basil:cross-outline" class="text-3xl"></iconify-icon>
          </button>

        </div>
      {/each}
      <button class="btn btn-outline join-item" onclick={() => addConversation()} aria-label="addConversation">
        <iconify-icon class="text-3xl" icon="mdi:plus"></iconify-icon>
      </button>
    </div>

    <br>
    {#if error.length > 0}
      <p class="text-error">Error: {error}</p>
    {/if}

  </div>

</div>
