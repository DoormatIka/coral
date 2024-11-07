
<script lang="ts">
  import type {HTMLInputTypeAttribute} from "svelte/elements";

  let { label, name, type, step, placeholder }: { label: string, name: string, type: HTMLInputTypeAttribute, step?: string, placeholder: string } = $props();
</script>

<div class="w-full flex items-center justify-center">
  <label class="w-full input input-bordered flex items-center gap-2 flex-auto">
    {label}
    <input name={name} type={type} step={step} placeholder={placeholder} class="grow" />
  </label>
  <slot></slot>
</div>
