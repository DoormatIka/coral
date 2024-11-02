<script lang="ts">
  import type { PageData } from './$types';
  import {onMount} from "svelte";
  import {fly} from "svelte/transition";
  import {invoke} from "@tauri-apps/api/core";
  import type {Message, MessageRequest} from '../../types';

  let { data }: { data: PageData } = $props();
  let chats: Message[] = $state(data.conversation.log);

  let chatelement: HTMLElement;
  let sendbutton: HTMLButtonElement;
  onMount(async () => {
    chatelement = document.getElementById("chat")!;
    sendbutton = document.getElementById("sendbutton")! as HTMLButtonElement;

    setTimeout(() => { chatelement.scrollBy({top: 99999, behavior: "smooth"}) }, 500);
  });

  // user input stuff
  
  let user_input = $state("");
  function showEditModal() {
    const my_modal_2 = document.getElementById("editmodal")! as any;
    my_modal_2.showModal();
  }

  let selected_message: number = $state(0);
  let user_edit_msg: string = $state("");
  function editMessage() {
    const previous_message = chats[selected_message];
    chats[selected_message] = structuredClone({person: previous_message.person, content: user_edit_msg});
    selected_message = 0;
    user_edit_msg = "";
  }

  async function regenMessage(index: number) {
    const user_msg = chats.splice(index)[0];
    await sendMessage(user_msg.content, true);
  }
  
  async function sendMessage(message: string, regen: boolean) {
    if (message.trim() !== "") {
      //                includes sys msg                 21 in length.
      const api_chat = chats.length <= 21 ? chats : [chats[0], ...chats.slice(-20).filter((_, i) => i > 0)];

      const apisend: MessageRequest = {
        "memory_id": "c9732cf2-c734-47cf-89f1-95a9cbd1e3b7-sato-chat",
        "log": api_chat,
        "regen": regen,
        "memories": api_chat.slice(-22, -23),
      }

      chats.push({person: "user", content: structuredClone(message)});
      user_input = "";
      setTimeout(() => { chatelement.scrollBy({top: 99999, behavior: "smooth"}) }, 100);

      sendbutton.disabled = true;

      try {
        const rawMsg: string = await invoke("create_ai_message", {conversationjson: JSON.stringify(apisend)});
        const msg: {message: string} = JSON.parse(rawMsg);
        chats.push({ person: "assistant", content: msg.message.trim()});
        sendbutton.disabled = false;
      } catch (err) {
        chats.pop();
        user_input = err as string;
        sendbutton.disabled = false;
      }

      setTimeout(() => { chatelement.scrollBy({top: 99999, behavior: "smooth"}) }, 100);
    }
  }
</script>


<dialog id="editmodal" class="modal overflow-hidden">
  <div class="modal-box max-w-full bg-base-200">
    <form method="dialog" class="sm:block hidden">
      <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">X</button>
    </form>
    <h3 class="text-2xl font-bold mb-3">Edit your message:</h3>
  <textarea class="textarea resize-y w-full" bind:value={user_edit_msg}></textarea>

    <form method="dialog">
      <button class="bg-neutral btn p-3 w-full mt-2" onclick={editMessage}>Set</button>
    </form>
  </div>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>

<!-- Chat Box -->
<div class="flex flex-col items-center overflow-hidden h-full">
  <div class="p-2 overflow-y-auto flex-1 w-full" id="chat">
    {#each chats as msg, index (msg)}
      <div class="w-full" transition:fly={{ duration: 400, x: -200 }}>

        {#if msg.person === "system"}
          <div class="chat flex justify-center items-center">
            <div class="chat-bubble whitespace-pre-line">{msg.content}</div>
          </div>
        {/if}
        {#if msg.person === "assistant"}
          <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
          <div class="dropdown dropdown-bottom w-full">
            <div class="chat chat-start">
              <div role="button" tabindex="0" class="chat-bubble relative whitespace-pre-line">{msg.content}</div>
              <div tabindex="0" class="dropdown-content bg-base-100 rounded-box z-[1] w-fit shadow">
                <button class="p-3" onclick={() => { selected_message = index; user_edit_msg = msg.content; showEditModal(); }}>Edit</button>
              </div>
            </div>
          </div>
        {/if}

        {#if msg.person === "user" && msg.content.length > 0}
          <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
          <div class="dropdown dropdown-bottom w-full">
            <div class="chat chat-end">
              <div role="button" tabindex="0" class="chat-bubble relative whitespace-pre-line">{msg.content}</div>
              <div tabindex="0" class="dropdown-content bg-base-100 rounded-box z-[1] w-fit shadow">
                <button class="p-3" onclick={() => {chats.splice(index)}}>Delete</button>
                <button class="p-3" onclick={() => {regenMessage(index)}}>Regen</button>
              </div>
            </div>
          </div>
        {/if}

      </div>
    {/each}
  </div>

  <div class="flex w-full items-center">
    <textarea class="textarea resize-none w-full" placeholder="Send a message..." bind:value={user_input}></textarea>
      <button class="btn h-full" id="sendbutton" onclick={() => {sendMessage(user_input, false)}} aria-label="Send Message">
      <iconify-icon icon="mynaui:send" class="text-3xl"></iconify-icon>
    </button>
  </div>

</div>
