
<script lang="ts">
  import Input from '$lib/Input.svelte';
  import type { PageData } from './$types';
  import {goto} from "$app/navigation";
  import {invoke} from "@tauri-apps/api/core";
  import {onMount} from 'svelte';

  let { data }: { data: PageData } = $props();
  const form_entries = [
    "link",
    "temp",
    "topP",
    "topK",
    "minP",
    "typicalP",
    "repeatPenalty",
    "tfsZ",
    "mirostatMode",
    "mirostatTau",
    "mirostatEta"
  ];
  onMount(() => {
    const filtered_data = Object.entries(data.settings).filter(element => form_entries.includes(element[0]));
    for (let i = 0; i < filtered_data.length; i++) {
      const element = document.getElementsByName(filtered_data[i][0])[0] as HTMLInputElement;
      element.value = filtered_data[i][1].toString();
    }
  });

  let err = $state("");
  async function setSettingsToDatabase(event: Event) {
    const formEl = event.target as HTMLFormElement;
    const data = new FormData(formEl);

    const filtered_data = data.entries().filter(element => form_entries.includes(element[0])).toArray();
    const args = new Map<string, string | number>()
    for (let i = 0; i < filtered_data.length; i++) {
      const data = filtered_data[i][1].toString();
      const try_conversion = Number(filtered_data[i][1].toString());

      if (filtered_data[i][0] === "link" || isNaN(try_conversion)) {
        args.set(filtered_data[i][0], data);
      } else {
        args.set(filtered_data[i][0], try_conversion);
      }

    }

    try {
      await invoke("change_settings", Object.fromEntries(args));
      await goto("/");
    } catch (inerr) {
      err = inerr as string;
    }
  }

</script>

<div class="h-full overflow-y-auto p-5">
  <h2 class="text-4xl">Settings</h2>
  <form onsubmit={setSettingsToDatabase} class="py-3 gap-3 flex flex-col lg:grid lg:grid-cols-2">

    <Input label="Link" name="link" type="text" placeholder="127.0.0.1"></Input>
    <Input label="Temp" name="temp" type="number" placeholder="0.1"></Input>
    <Input label="Top p" name="topP" type="number" placeholder="0.1"></Input>
    <Input label="Top k" name="topK" type="number" placeholder="0.1"></Input>
    <Input label="Min p" name="minP" type="number" placeholder="0.1"></Input>
    <Input label="Typical p" name="typicalP" type="number" placeholder="0.1"></Input>
    <Input label="Repeat Penalty" name="repeatPenalty" type="number" placeholder="0.1"></Input>
    <Input label="tfs_z" name="tfsZ" type="number" placeholder="0.1"></Input>
    <Input label="Mirostat Mode" name="mirostatMode" type="number" placeholder="0.1"></Input>
    <Input label="Mirostat Tau" name="mirostatTau" type="number" placeholder="0.1"></Input>
    <Input label="Mirostat Eta" name="mirostatEta" type="number" placeholder="0.1"></Input>

    {#if err.length > 0}
      <p class="text-error">Error: {err}</p>
    {/if}
    <button type="submit" class="mt-5 w-full btn btn-neutral">Submit</button>
  </form>
</div>
