<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";

const enabled = ref(false);
const title = computed(() =>
  enabled.value ? "4K 녹화 가이드 끄기" : "4K 녹화 가이드 켜기",
);

function readGuideState() {
  enabled.value =
    new URLSearchParams(window.location.search).get("guide") === "4k";
}

function toggleGuide() {
  const url = new URL(window.location.href);

  if (enabled.value) {
    url.searchParams.delete("guide");
  } else {
    url.searchParams.set("guide", "4k");
  }

  window.history.replaceState(window.history.state, "", url);
  window.dispatchEvent(new PopStateEvent("popstate"));
}

onMounted(() => {
  readGuideState();
  window.addEventListener("popstate", readGuideState);
});

onBeforeUnmount(() => {
  window.removeEventListener("popstate", readGuideState);
});
</script>

<template>
  <div class="recording-guide-toggle__divider" />
  <button
    class="slidev-icon-btn recording-guide-toggle"
    :class="{ active: enabled }"
    :title="title"
    :aria-label="title"
    :aria-pressed="enabled"
    type="button"
    @click="toggleGuide"
  >
    <span class="recording-guide-toggle__label">4K</span>
  </button>
</template>

<style>
.recording-guide-toggle__divider {
  background: currentcolor;
  margin: 0.25rem 0.5rem;
  opacity: 0.1;
  width: 1px;
}

.recording-guide-toggle__label {
  font:
    800 0.58em/1 "SFMono-Regular",
    "Cascadia Code",
    Menlo,
    monospace;
  letter-spacing: -0.08em;
  padding-right: 0.08em;
}

.recording-guide-toggle.active {
  color: #55d6c2;
}
</style>
