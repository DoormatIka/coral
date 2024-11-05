
<script lang="ts">
  import type { PageData } from './$types';
  import {goto} from "$app/navigation";
  import {invoke} from "@tauri-apps/api/core";

  let { data }: { data: PageData } = $props();

  async function setSettingsToDatabase(event: Event) {
    const formEl = event.target as HTMLFormElement;
    const data = new FormData(formEl);

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
    const filtered_data = data.entries().filter(element => form_entries.includes(element[0])).toArray();
    const args = new Map<string, string>()
    for (let i = 0; i < filtered_data.length; i++) {
      args.set(filtered_data[i][0], filtered_data[i][1].toString());
    }

    try {
      await invoke("change_settings", Object.fromEntries(args));
      await goto("/");
    } catch (inerr) {
      console.log(inerr);
      // err = inerr as string;
      // showErrorModal();
    }
  }

</script>

<div class="h-full overflow-y-auto p-5">
  <h2 class="text-4xl">Settings</h2>
  <div class="py-3 gap-3 flex flex-col lg:grid lg:grid-cols-2">
    <label class="input input-bordered flex items-center gap-2">
      Link
      <input name="link" type="text" placeholder="128.0.0.1" />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      Temp
      <input name="temp" type="text" placeholder="0.1" />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      Top p
      <input name="top_p" type="text" placeholder="0.1" />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      Top k
      <input name="top_k" type="text" placeholder="0.1" />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      Min p
      <input name="min_p" type="text" placeholder="0.1" />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      Typical p
      <input name="typical_p" type="text" placeholder="0.1" />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      Repeat Penalty
      <input name="repeat_penalty" type="text" placeholder="0.1" />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      tfs_z
      <input name="tfs_z" type="text" placeholder="0.1" />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      Mirostat Mode
      <input name="mirostat_mode" type="text" placeholder="0.1" />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      Mirostat Tau
      <input name="mirostat_tau" type="text" placeholder="0.1" />
    </label>
    <label class="input input-bordered flex items-center gap-2">
      Mirostat Eta
      <input name="mirostat_eta" type="text" placeholder="0.1" />
    </label>
  </div>
  <button type="submit" on:click={setSettingsToDatabase} class="mt-5 w-full btn btn-neutral">Submit</button>
</div>
