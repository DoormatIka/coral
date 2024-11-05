
<script lang="ts">
  let { 
    link,
    image_link, 
    title, 
    description,
    removeCharacter,
  }: { link: string, image_link: string, title: string, description: string, removeCharacter: () => Promise<void> } = $props();
</script>

<div class="card bg-base-100 image-full shadow-xl h-full">

  <figure>
    <img
      class="object-cover object-top w-full"
      src={image_link}
      alt="Shoes" />
  </figure>

  <div class="card-body min-w-full h-full p-3">

    <div class="card-actions justify-end z-50 w-full">
      <button class="btn btn-primary btn-sm" onclick={async () => { await removeCharacter() }} aria-label="close">
        <iconify-icon icon="basil:cross-outline" class="text-2xl"></iconify-icon>
      </button>
    </div>
    <a class="p-3 h-full" href={link} data-sveltekit-preload-data="off">
      <div class="card-title">
        <h2 class="truncate">{title}</h2>
      </div>
      <div class="text-ellipsis overflow-hidden">
        <p>{description}</p>
      </div>
    </a>

  </div>

</div>
