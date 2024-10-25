
<script lang="ts">
  import {onMount} from "svelte";

  let c: HTMLElement;
  onMount(() => {
    c = document.getElementById("chat")!;
    setTimeout(() => { c.scrollBy({top: 99999, behavior: "smooth"}) }, 500);
  })

  let message = $state("");
  const chats = $state([
    {user: "user", content: "Hello."},
    {user: "assistant", content: "Hey there."},
    {user: "assistant", content: "Here's a second message as a jumpscare."},
    {user: "user", content: "wtf."},
    {user: "assistant", content: "The Mistress of the Palace of the Earth Spirits after the underground city was separated from Hell. Her ability to read minds causes various youkai and spirits to fear her, but makes her loved by the animals that normally can't be understood. With these animals as pets, she manages the ruins of the Hell of Blazing Fires where her home stands. She is also the main protagonist of Foul Detective Satori. "},
    {user: "user", content: "sabi mo eh."},
    {user: "assistant", content: "The Mistress"},
    {user: "assistant", content: "of the Palace of the Earth Spirits"},
  ]);
  for (let i = 0; i < 999; i++) {
    const max = 1;
    const min = 0;
    const us = Math.floor(Math.random() * (max - min + 1) + min);
    const a: ("user" | "assistant")[] = ["user", "assistant"]
    chats.push({user: a[us], content: `${i} item`});
  }
  
  function sendMessage() {
    if (message.trim() !== "") {
      chats.push({user: "user", content: structuredClone(message)})
      message = "";
      setTimeout(() => { c.scrollBy({top: 99999, behavior: "smooth"}) }, 100);
    }
  }
</script>

<!-- Chat Box -->
<div class="flex flex-col items-center overflow-hidden h-full">

  <div class="p-5 overflow-y-auto flex-1" id="chat">
    {#each chats as msg}
      {#if msg.user === "assistant"}
        <div class="chat chat-start">
          <div class="chat-bubble max-w-fit sm:w-3/4">{msg.content}</div>
        </div>
      {/if}
      {#if msg.user === "user"}
        <div class="chat chat-end">
          <div class="chat-bubble max-w-fit sm:w-3/4">{msg.content}</div>
        </div>
      {/if}
    {/each}
  </div>

  <div class="flex w-full items-center">
    <textarea class="textarea resize-none w-full" placeholder="Send a message..." bind:value={message}></textarea>
    <button class="btn h-full" on:click={sendMessage}>
      <iconify-icon icon="mynaui:send" class="text-3xl"></iconify-icon>
    </button>
  </div>

</div>
