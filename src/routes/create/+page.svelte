
<script lang="ts">
  import {open} from "@tauri-apps/plugin-dialog";
  import {goto} from "$app/navigation";
  import {invoke} from "@tauri-apps/api/core";
  import {readFile} from '@tauri-apps/plugin-fs';
  import {base} from '$app/paths'

  const image_link = `${base}/no_image.png`;
  let selected_image: string | null = null;
  let err: string = "";

  function showErrorModal() {
    (document.getElementById("errormodal")! as any).showModal();
  }
  async function openDialog() {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: 'Image',
        extensions: ['png', 'jpeg']
      }],
    });
    if (!file) { return; }
    
    const filePath = file as string;

    const selected_image_buffer = await readFile(filePath);

    // Convert binary buffer to data URL
    const blob = new Blob([selected_image_buffer]);
    const reader = new FileReader();
    reader.onload = (event) => {
      selected_image = event.target?.result as string;
    };
    reader.readAsDataURL(blob);
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
      // Add Character to Database
      await invoke("add_character", { // first_message (in rust) = firstMessage (in JS) :(
        name: name,
        description: description,
        firstMessage: first_message,
        systemMessage: system_message,
        imageBuffer: selected_image ?? image_link,
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

<form onsubmit={addCharacterToDatabase} class="h-full w-full p-5 overflow-y-auto m-auto">
  <h2 class="text-4xl py-3">Create Character</h2>

  <div class="flex flex-col justify-center items-center gap-2 join join-vertical">
    <div class="avatar join-item">
      <div class="w-32 join-item">
        <img src={selected_image ?? image_link} alt="nagakuuu" />
      </div>
    </div>
    <button type="button" onclick={openDialog} class="btn-outline btn rounded w-32 join-item">Select Avatar</button>
  </div>

  <input type="text" name="name" class="mt-3 input input-bordered w-full" placeholder="Name" required />
  <textarea name="description" class="mt-3 w-full textarea textarea-bordered min-h-32" placeholder="Description" required></textarea>
  <textarea name="first_message" class="mt-3 w-full textarea textarea-bordered min-h-32" placeholder="First Message" required></textarea>
  <textarea name="system_message" class="mt-3 w-full textarea textarea-bordered min-h-32" placeholder="System Message" required></textarea>
  <button type="submit" class="mt-5 w-full btn btn-neutral">Submit</button>
</form>
