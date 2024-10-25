
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
  <div class="h-full w-full flex">
    <div class="min-w-max border-r border-neutral overflow-y-auto sm:block hidden">
      <div class="p-5">
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
    <!-- main -->
    <slot />
  </div>
</main>
