
<script>
	import { page } from '$app/stores';
</script>

<div class="p-5">
  <h1>{$page.status}</h1> 
  <p>{$page.error?.message}</p>
</div>
