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
  const chats: { user: "user" | "assistant", content: string }[] = $state([
    {user: "user", content: "Hello."},
    {user: "assistant", content: "Hey there."},
    {user: "assistant", content: "Here's a second message as a jumpscare."},
    {user: "user", content: "wtf."},
    {user: "assistant", content: "The Mistress of the Palace of the Earth Spirits after the underground city was separated from Hell. Her ability to read minds causes various youkai and spirits to fear her, but makes her loved by the animals that normally can't be understood. With these animals as pets, she manages the ruins of the Hell of Blazing Fires where her home stands. She is also the main protagonist of Foul Detective Satori. "},
    {user: "user", content: "sabi mo eh."},
    {user: "assistant", content: "of the Palace of the Earth Spirits"},
  ]);
  
  async function sendMessage() {
    if (message.trim() !== "") {
      chats.push({user: "user", content: structuredClone(message)});
      const greetMsg: string = await invoke("greet", { name: "Alice" });
      chats.push({user: "assistant", content: greetMsg});
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

        {#if msg.user === "assistant"}
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

        {#if msg.user === "user"}
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
