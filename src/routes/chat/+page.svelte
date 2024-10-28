<script lang="ts">
  import {onMount} from "svelte";
  import {fly} from "svelte/transition";
  import {invoke} from "@tauri-apps/api/core";

  let c: HTMLElement;
  onMount(() => {
    c = document.getElementById("chat")!;
    setTimeout(() => { c.scrollBy({top: 99999, behavior: "smooth"}) }, 500);
  });
  let message = $state("");
  const chats: { person: "system" | "user" | "assistant", content: string }[] = $state([
    {person: "system", content: "[INST]Act like Parsee Mizuhashi from Touhou Project, very obsessive and jealous. Also keep your responses short but detailed at the same time.[/INST]"},
  ]);
  const apisend = {
    "memory_id": "c9732cf2-c734-47cf-89f1-95a9cbd1e3b7-sato-chat",
    "log": chats,
    "regen": false,
  }
  
  async function sendMessage() {
    if (message.trim() !== "") {
      chats.push({person: "user", content: structuredClone(message)});
      let greetMsg: string = "";
      try {
        greetMsg = await invoke("create_ai_message", {conversationjson: JSON.stringify(apisend)});
      } catch (err) {
        greetMsg = err as string;
      }
      chats.push({person: "assistant", content: greetMsg});
      message = "";
      setTimeout(() => { c.scrollBy({top: 99999, behavior: "smooth"}) }, 100);
    }
  }
</script>

<!-- Chat Box -->
<div class="flex flex-col items-center overflow-hidden h-full">

  <div class="p-2 overflow-y-auto flex-1 w-full" id="chat">
    {#each chats as msg, index (msg)}
      <div class="w-full" transition:fly={{ duration: 400, x: -200 }}>

        {#if msg.person === "assistant"}
          <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
          <div class="dropdown dropdown-bottom w-full">
            <div class="chat chat-start">
              <div role="button" tabindex="0" class="chat-bubble relative">{msg.content}</div>
              <div tabindex="0" class="dropdown-content bg-base-100 rounded-box z-[1] w-fit shadow">
                <button class="p-3" onclick={() => { chats.splice(index); }}>Delete</button>
              </div>
            </div>
          </div>
        {/if}

        {#if msg.person === "user"}
          <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
          <div class="dropdown dropdown-bottom w-full">
            <div class="chat chat-end">
              <div role="button" tabindex="0" class="chat-bubble relative">{msg.content}</div>
              <div tabindex="0" class="dropdown-content bg-base-100 rounded-box z-[1] w-fit shadow">
                <button class="p-3" onclick={() => { chats.splice(index); }}>Delete</button>
              </div>
            </div>
          </div>
        {/if}

      </div>
    {/each}
  </div>

  <div class="flex w-full items-center">
    <textarea class="textarea resize-none w-full" placeholder="Send a message..." bind:value={message}></textarea>
    <button class="btn h-full" onclick={sendMessage} aria-label="Send Message">
      <iconify-icon icon="mynaui:send" class="text-3xl"></iconify-icon>
    </button>
  </div>

</div>
