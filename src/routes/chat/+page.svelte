
<script lang="ts">
  const chats: {user: "assistant" | "user", content: string}[] = [
    {user: "user", content: "Hello."},
    {user: "assistant", content: "Hey there."},
    {user: "assistant", content: "Here's a second message as a jumpscare."},
    {user: "user", content: "wtf."},
    {user: "assistant", content: "The Mistress of the Palace of the Earth Spirits after the underground city was separated from Hell. Her ability to read minds causes various youkai and spirits to fear her, but makes her loved by the animals that normally can't be understood. With these animals as pets, she manages the ruins of the Hell of Blazing Fires where her home stands. She is also the main protagonist of Foul Detective Satori. "},
    {user: "user", content: "sabi mo eh."}
  ];
  // need to accept chats
</script>

<!-- Chat Box -->
<div class="flex flex-col w-full h-full items-center overflow-hidden">
  <div class="w-full border-b border-neutral flex flex-row content-between">
    <div class="flex-1 sm:hidden block">
      <iconify-icon icon="mdi:hamburger-menu" class="p-3 text-3xl" style="vertical-align: -0.125em;"></iconify-icon>
    </div>
    <a href="/" class="m-3">
      <iconify-icon icon="weui:back-filled" class="text-3xl" style="vertical-align: -0.125em;"></iconify-icon>
    </a>
  </div>

  <div class="flex-1 w-full p-5 overflow-y-auto">
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
    <textarea class="textarea resize-none w-full" placeholder="Send a message..."></textarea>
    <button class="btn h-full"><iconify-icon icon="mynaui:send" class="text-3xl"></iconify-icon></button>
  </div>
</div>
