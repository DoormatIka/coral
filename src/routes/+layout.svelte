<script lang="ts">
  import "../app.css";
  import Avatar from "$lib/Avatar.svelte";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";

  let isOnline = $state(true);

  let current_link = $state("10.147.18.27");
  const avatars = [
    "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp",
    "https://cdn.discordapp.com/attachments/914427100935647245/1298626589063381073/image.png?ex=671ae882&is=67199702&hm=09d4844da579a73b73180d3fa1b3f8b2fa93cbcf32c57f0ce17202e2ae21bcc9&",
    "https://cdn.discordapp.com/attachments/914427100935647245/1298618003021365309/FB_IMG_1729685032217.jpg?ex=671ae083&is=67198f03&hm=6587407ef9e9f3e65999552e0130877fa3e3b9e441941c3f73666066ce3276be&",
  ];

  function showIPModal() {
    const my_modal_2 = document.getElementById("my_modal_2")! as any;
    my_modal_2.showModal();
  }
  // getLink function...
  async function changeLink() {
    await invoke("change_link", { link: current_link });
  }
  async function linkSanityCheck() {
    try {
      await fetch("http://" + current_link + "/ping", {
        method: "GET",
        mode: "no-cors",
      });
      isOnline = true;
    } catch (err) {
      isOnline = false;
    }
  }

</script>


<main>
  <div class="drawer sm:drawer-open w-screen">
    <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />

    <div class="drawer-content flex flex-col h-screen">

      <!-- Top -->
      <div class="w-full border-b border-neutral flex flex-row content-between gap-3 px-3">
        <label for="my-drawer-2" class="flex-1 sm:hidden block">
          <iconify-icon icon="mdi:hamburger-menu" class="py-3 text-3xl" style="vertical-align: -0.125em;"></iconify-icon>
        </label>

        {#if $page.route.id !== "/"}
          <a href="/" class="flex items-center px-3" aria-label="Home">
            <iconify-icon icon="weui:back-filled" class="text-3xl py-3" style="vertical-align: -0.125em;"></iconify-icon>
          </a>
        {:else}
          <button onclick={showIPModal} aria-label="id">
            <iconify-icon icon="mdi:ip" class="text-3xl py-3" style="vertical-align: -0.125em;"></iconify-icon>
          </button>
          <button onclick={linkSanityCheck} class="btn btn-outline">Sanity Check: {isOnline ? "Sane" : "Insane"}</button>
        {/if}
      </div>

      <!-- Link Modal -->
      <dialog id="my_modal_2" class="modal overflow-hidden">
        <div class="modal-box bg-base-200">
          <form method="dialog" class="sm:block hidden">
            <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">X</button>
          </form>
          <h3 class="text-2xl font-bold mb-3">Set your IP here.</h3>
          <label class="input input-bordered flex items-center gap-2">
            <iconify-icon icon="mdi:ip" class="text-3xl" style="vertical-align: -0.125em;"></iconify-icon>
            <input bind:value={current_link} type="text" placeholder="127.0.0.1" />
          </label>

          <form method="dialog">
            <button class="bg-neutral btn p-3 w-full mt-2" onclick={changeLink}>Set</button>
          </form>
        </div>
        <form method="dialog" class="modal-backdrop">
          <button>close</button>
        </form>
      </dialog>

      <!-- Page content here -->
      <slot />

    </div>

    <!-- Drawer -->
    <div class="drawer-side z-50">
      <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label>
      <div class="p-5 min-h-full bg-base-100 border-r border-neutral">

        <!-- Side bar. -->
        <h1>Coral</h1>
        <h2 class="my-2">History</h2>
        <div class="content-center grid grid-cols-2 gap-2">
          {#each avatars as avatar}
            <a href="/chat">
              <div class="w-14">
                <Avatar avatar={avatar}/>
              </div>
            </a>
          {/each}
        </div>
        <br>
        <a href="/create" class="btn w-full">Create</a>


      </div>
    </div>
  </div>
</main>
