<script lang="ts">
  let { children } = $props();
  import "../app.css";
  let tab1Active = $state(true);
  let tab2Active = $state(false);
  let tab3Active = $state(false);

  function handleTabClick(_event: MouseEvent & { currentTarget: EventTarget & HTMLAnchorElement }, type: number) {
    tab1Active = type === 0b100;
    tab2Active = type === 0b010;
    tab3Active = type === 0b001;
  }
</script>

<div class="h-screen w-screen bg-cyan-900 p-5 flex flex-col">
  <div class="h-1/12 w-full">
    <div role="tablist" class="tabs tabs-border">
      <a
        href="/guid-converter"
        role="tab"
        class={["tab", tab1Active && "tab-active"]}
        onclick={(event) => handleTabClick(event, 0b100)}><p class="text-lg font-bold">Guid Converter</p></a
      >
      <a
        href="/json-parser"
        role="tab"
        class={["tab", tab2Active && "tab-active"]}
        onclick={(event) => handleTabClick(event, 0b010)}><p class="text-lg font-bold">Json Parser</p></a
      >
      <a
        href="/angular-builder"
        role="tab"
        class={["tab", tab3Active && "tab-active"]}
        onclick={(event) => handleTabClick(event, 0b001)}><p class="text-lg font-bold">Angular Builder</p></a
      >
    </div>
  </div>
  <div class="h-11/12 w-full">
    {@render children()}
  </div>
</div>
