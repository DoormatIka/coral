
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
    <div class="card-title">
      <h2 class="truncate">{title}</h2>
    </div>
    <div class="text-ellipsis overflow-hidden">
      <p>{description}</p>
    </div>
  </div>
</div>
