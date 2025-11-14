<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let inputKey = $state();
  let inputJson = $state();

  async function handleFindClick() {
    console.log(inputKey);
    console.log(inputJson);

    let result = await await invoke("parse_and_find_json", { inputKey, inputJson });
    console.log(result);
  }
</script>

<div class="w-full h-full flex flex-col gap-4">
  <div class="w-full flex flex-row gap-4">
    <p class="w-[30%]">Insert specific key that you want to find (case sensitive)</p>
    <input bind:value={inputKey} type="text" placeholder="Insert your key here" class="input w-[40%]" />
    <button class="btn btn-primary" onclick={() => handleFindClick()} disabled={inputJson == null || inputJson == ""}
      >Find</button
    >
  </div>
  <div class="w-full h-full flex flex-row gap-4">
    <div class="w-1/2 h-full flex flex-col">
      <p>Input your JSON here</p>
      <div class="h-full w-full">
        <textarea bind:value={inputJson} class="textarea h-full w-full overflow-auto"></textarea>
      </div>
    </div>
    <div class="w-1/2 h-full flex flex-col">
      <p>Input your JSON here</p>
      <div class="h-full w-full">
        <textarea class="textarea h-full w-full overflow-auto" disabled></textarea>
      </div>
    </div>
  </div>
</div>

<style>
  textarea {
    resize: none;
  }
</style>
