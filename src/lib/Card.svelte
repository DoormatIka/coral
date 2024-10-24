
<script>
  export let link = "";
  export let title = "";
  export let description = "";
</script>

<div class="card bg-base-100 image-full shadow-xl h-60">
  <figure>
    <img
      class="object-cover object-top w-full"
      src={link}
      alt="Shoes" />
  </figure>
  <div class="card-body min-w-full min-h-full">
    <h2 class="card-title">{title}</h2>
    <p class="text-ellipsis overflow-hidden">{description}</p>
  </div>
</div>
