
<script lang="ts">
  import type { CharacterConversation } from "../../types";
  const conversations: CharacterConversation[] = [
    {"from_char_id": "abc", "id": "dsadas", "log": [{"person": "system", "content": "message 1"}, {"person": "system", "content": "message 2"}, {"person": "system", "content": "message 3"}], "memory_id": ""},
    {"from_char_id": "abc", "id": "dsadasa", "log": [{"person": "system", "content": "message 1"}, {"person": "system", "content": "message 2"}, {"person": "system", "content": "message 3"}], "memory_id": ""},
    {"from_char_id": "abc", "id": "dsadasbb", "log": [{"person": "system", "content": "message 1"}, {"person": "system", "content": "message 2"}, {"person": "system", "content": "message 3dsajhdg asuidhasioudhasoidf hasfoimdshfodshf oiusdhfosdhf iodhfoisdfo scjisdfo".repeat(100)}], "memory_id": ""},
  ];
</script>

<div class="w-full h-full join join-vertical p-5">
  {#each conversations as conversation, index}
    <a href="/" class="join-item border border-sm border-neutral hover:bg-neutral transition-colors p-5 max-h-20 truncate text-ellipsis">
      Conversation {index + 1}: {conversation.log.at(-1)!.content}
    </a>
  {/each}
</div>
