
<script lang="ts">
  import {goto} from "$app/navigation";
  import {invoke} from "@tauri-apps/api/core";

  let err: string = "";
  function showErrorModal() {
    (document.getElementById("errormodal")! as any).showModal();
  }

  async function addCharacterToDatabase(event: Event) {
    err = "";
    const formEl = event.target as HTMLFormElement;
    const data = new FormData(formEl);

    const name = data.get("name");
    const first_message = data.get("first_message");
    const system_message = data.get("system_message");
    const description = data.get("description");

    try {
      await invoke("add_character", { // first_message (in rust) = firstMessage (in JS) :(
        name: name,
        description: description,
        firstMessage: first_message,
        systemMessage: system_message,
      });
      await goto("/");
    } catch (inerr) {
      err = inerr as string;
      showErrorModal();
    }
  }
</script>

<dialog id="errormodal" class="modal overflow-hidden">
  <div class="modal-box bg-base-200">
    <form method="dialog" class="sm:block hidden">
      <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">X</button>
    </form>
    <h2>Error:</h2>
    <p>{err}</p>
  </div>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>

<div class="flex flex-col items-center overflow-hidden h-full">
  <form onsubmit={addCharacterToDatabase} class="flex-col flex h-full items-center justify-center gap-3 w-1/2">
    <h2>Create Character</h2>
    <input type="text" name="name" class="input input-bordered w-full" placeholder="Name" required />
    <textarea name="description" class="w-full textarea textarea-bordered" placeholder="Description" required></textarea>
    <textarea name="first_message" class="w-full textarea textarea-bordered" placeholder="First Message" required></textarea>
    <textarea name="system_message" class="w-full textarea textarea-bordered" placeholder="System Message" required></textarea>
    <button type="submit" class="w-full btn btn-neutral">Submit</button>
  </form>
</div>
