
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
    "top_p",
    "top_k",
    "min_p",
    "typical_p",
    "repeat_penalty",
    "tfs_z",
    "mirostat_mode",
    "mirostat_tau",
    "mirostat_eta"
  ];
  onMount(() => {
    const filtered_data = Object.entries(data.settings).filter(element => form_entries.includes(element[0]));
    for (let i = 0; i < filtered_data.length; i++) {
      if (filtered_data[i][0] === "mirostat_mode") {
        const element = document.getElementsByName(filtered_data[i][0])[0] as HTMLSelectElement;
        element.selectedIndex = Number(filtered_data[i][1]);
      } else {
        const element = document.getElementsByName(filtered_data[i][0])[0] as HTMLInputElement;
        element.value = filtered_data[i][1].toString();
      }
    }
  });

  let err = $state("");
  async function setSettingsToDatabase(event: Event) {
    const formEl = event.target as HTMLFormElement;
    const data = new FormData(formEl);

    const filtered_data = data.entries().filter(element => form_entries.includes(element[0])).toArray();
    const args = new Map<string, string | number>()
    for (let i = 0; i < filtered_data.length; i++) {
      // snakeToCamel due to tauri not recognizing snake_case variables.
      const name = snakeToCamel(filtered_data[i][0]);
      const data = filtered_data[i][1].toString();
      const try_conversion = Number(filtered_data[i][1].toString());

      if (name === "link") {
        args.set(name, data);
        continue;
      }
      if (name === "mirostatMode") {
        args.set(name, mirostatModeToNumber(data));
        continue;
      }
      if (isNaN(try_conversion)) {
        args.set(name, data);
      } else {
        args.set(name, try_conversion);
      }
    }

    try {
      await invoke("change_settings", Object.fromEntries(args));
      await goto("/");
    } catch (inerr) {
      err = inerr as string;
    }
  }
  function mirostatModeToNumber(text: string) {
    switch (text) {
      case "Disabled":
        return 0;
      case "Mirostat":
        return 1;
      case "Mirostat 2.0":
        return 2;
      default:
        return 0;
    }
  }
  function snakeToCamel(str: string) {
    return str.toLowerCase().replace(/([-_][a-z])/g, group =>
      group
        .toUpperCase()
        .replace('-', '')
      .replace('_', '')
    )
  }

  let isOnline = $state(true);
  let inProgress = $state(false);
  async function linkSanityCheck() {
    const element = document.getElementsByName("link")[0] as HTMLInputElement;
    inProgress = true;
    await invoke("sanity_check", { link: element.value })
      .then(() => {
        isOnline = true;
      })
      .catch(() => {
        isOnline = false;
      })
      .finally(() => {
        inProgress = false;
      });
  }
</script>

<div class="h-full overflow-y-auto p-5">
  <h1 class="text-5xl">Settings</h1>
  <form onsubmit={setSettingsToDatabase} class="py-3">

    <div class="gap-3 flex flex-col lg:grid lg:grid-cols-2">
      <Input label="Link" name="link" type="text" placeholder="127.0.0.1">
        <button onclick={linkSanityCheck} class="ml-3 btn btn-outline" type="button">
          {#if inProgress}
            <span class="loading loading-spinner loading-sm"></span>
          {:else}
            {#if isOnline}
              <iconify-icon class="text-xl" icon="mdi:check-bold"></iconify-icon>
            {:else}
              <iconify-icon class="text-xl" icon="maki:cross"></iconify-icon>
            {/if}
          {/if}
        </button>
      </Input>

      <Input label="Temp" name="temp" type="number" step="0.01" placeholder="0.1"></Input>
      <Input label="Top p" name="top_p" type="number" step="0.01" placeholder="0.1"></Input>
      <Input label="Top k" name="top_k" type="number" step="1" placeholder="40"></Input>
      <Input label="Min p" name="min_p" type="number" step="0.001" placeholder="0.1"></Input>
      <Input label="Typical p" name="typical_p" type="number" step="0.01" placeholder="0.1"></Input>
      <Input label="Repeat Penalty" name="repeat_penalty" type="number" step="0.01" placeholder="0.1"></Input>
      <Input label="tfs_z" name="tfs_z" type="number" step="0.01" placeholder="0.1"></Input>
    </div>

    <br>

    <h2 class="text-3xl py-3">Mirostat</h2>
    <div class="gap-3 flex flex-col lg:grid lg:grid-cols-2">
      <select class="select select-bordered w-full" name="mirostat_mode">
        <option selected>Disabled</option>
        <option>Mirostat</option>
        <option>Mirostat 2.0</option>
      </select>
      <Input label="Mirostat Tau" name="mirostat_tau" type="number" step="0.01" placeholder="0.1"></Input>
      <Input label="Mirostat Eta" name="mirostat_eta" type="number" step="0.01" placeholder="0.1"></Input>
    </div>

    {#if err.length > 0}
      <p class="text-error">Error: {err}</p>
    {/if}
    <button type="submit" class="mt-5 w-full btn btn-neutral">Submit</button>
  </form>
</div>
