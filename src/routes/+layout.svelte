
<script>
  import "../app.css";
  import Avatar from "$lib/Avatar.svelte";

  // grab avatar from http rust here.
  const avatars = [
    "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp",
    "https://cdn.discordapp.com/attachments/914427100935647245/1298626589063381073/image.png?ex=671ae882&is=67199702&hm=09d4844da579a73b73180d3fa1b3f8b2fa93cbcf32c57f0ce17202e2ae21bcc9&",
    "https://cdn.discordapp.com/attachments/914427100935647245/1298618003021365309/FB_IMG_1729685032217.jpg?ex=671ae083&is=67198f03&hm=6587407ef9e9f3e65999552e0130877fa3e3b9e441941c3f73666066ce3276be&",
  ];
</script>


<main>
  <div class="drawer sm:drawer-open w-screen">
    <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />

    <div class="drawer-content flex flex-col h-screen">
      <div class="w-full border-b border-neutral flex flex-row content-between">
        <label for="my-drawer-2" class="flex-1 sm:hidden block">
          <iconify-icon icon="mdi:hamburger-menu" class="p-3 text-3xl" style="vertical-align: -0.125em;"></iconify-icon>
        </label>
        <a href="/" class="p-3">
          <iconify-icon icon="weui:back-filled" class="text-3xl" style="vertical-align: -0.125em;"></iconify-icon>
        </a>
      </div>
      <slot />
      <!-- Page content here -->

    </div>
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

      </div>
    </div>
  </div>
</main>
