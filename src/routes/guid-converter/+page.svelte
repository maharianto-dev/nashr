<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { animate } from "animejs";
  import type { BaseApiResult } from "../../models/api-result.model";
  import type { GuidResult } from "../../models/guid-converter.model";

  let inputGuid = $state();
  let convertDisabled = $derived(inputGuid == null || inputGuid === "");
  let resultGuid: GuidResult | null = $state();
  let copyToClipboard = $state(false);

  let labelMap: { [key: string]: [string] } = {
    guid_standard: "Standard",
    guid_bracketed: "Bracketed",
    guid_oracle_raw16_format: "Oracle RAW(16) format",
    guid_oracle_hextoraw: "Oracle hextoraw",
    guid_sqlserver_using_oracle_byte_order: "SQLServer using Oracle byte order",
  };

  function resetResult() {
    resultGuid = null;
  }

  async function handleConvertClick() {
    resetResult();
    const result: BaseApiResult<GuidResult> = await invoke("convert_guid", { guidInput: inputGuid });
    if (result.is_successful) {
      resultGuid = result.payload;
    }
  }

  async function handleGenerateClick() {
    resetResult();
    inputGuid = null;
    const result = await invoke("generate_guid");
    if (result.is_successful) {
      resultGuid = result.payload;
      inputGuid = resultGuid.guid_standard;
    }
  }

  async function handleCopyToClipboardClick(value: string, key?: string) {
    copyToClipboard = true;
    await navigator.clipboard.writeText(value);
    let cssClass = ".tooltip";
    if (key != null) {
      cssClass = `${cssClass}.${key}`;
    }
    await animate(cssClass, {
      x: "50px",
      opacity: [{ to: 1 }],
      duration: 1000,
    });
    copyToClipboard = false;
  }
</script>

<div class="w-full flex flex-col gap-4">
  <div class="flex flex-row w-full items-center gap-4">
    <p class="w-[20%]">Convert your GUID here!</p>
    <input bind:value={inputGuid} type="text" placeholder="Insert your guid here" class="input w-[40%]" />
    <button class="btn btn-primary" disabled={convertDisabled} onclick={() => handleConvertClick()}>Convert</button>
    <button class="btn btn-primary" onclick={() => handleGenerateClick()}>Generate</button>
  </div>
  {#if resultGuid != null && resultGuid != ""}
    <div class="flex flex-col w-full gap-4">
      {#each Object.keys(resultGuid) as key}
        <div class="flex flex-row w-full items-center gap-4">
          <p class="w-[20%]">{labelMap[key]}</p>
          <div class="join w-[40%]">
            <input bind:value={resultGuid[key]} type="text" class="input join-item w-full" disabled />
            <button class="btn btn-primary" onclick={() => handleCopyToClipboardClick(resultGuid[key], key)}
              ><svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-6"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M15.666 3.888A2.25 2.25 0 0 0 13.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 0 1-.75.75H9a.75.75 0 0 1-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 0 1-2.25 2.25H6.75A2.25 2.25 0 0 1 4.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 0 1 1.927-.184"
                />
              </svg>
            </button>
          </div>
          {#if copyToClipboard === true}
            <div class="tooltip {key} rounded-sm p-1 bg-green-300 text-black font-semibold">Copied to clipboard!</div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .tooltip {
    position: relative;
    left: -70px;
    opacity: 0;
    z-index: 9999 !important;
  }
</style>
