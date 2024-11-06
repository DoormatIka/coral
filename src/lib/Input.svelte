
<script lang="ts">
  import type {HTMLInputTypeAttribute} from "svelte/elements";

  let { label, name, type, placeholder }: { label: string, name: string, type: HTMLInputTypeAttribute, placeholder: string } = $props();
</script>

<label class="input input-bordered flex items-center gap-2">
  {label}
  <input name={name} type={type} placeholder={placeholder} class="grow" />
</label>
