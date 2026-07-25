<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from "vue";

const enabled = ref(false);
const guideElement = ref<HTMLElement | null>(null);
const slideWidth = ref(0);
const slideHeight = ref(0);
const outputWidth = ref(0);
const outputHeight = ref(0);
const devicePixelRatio = ref(1);
let resizeObserver: ResizeObserver | undefined;
let animationFrame = 0;

const targetSlideWidth = computed(() =>
  Math.round(3840 / devicePixelRatio.value),
);
const targetSlideHeight = computed(() =>
  Math.round(2160 / devicePixelRatio.value),
);
const isSixteenByNine = computed(() => {
  if (outputHeight.value === 0) {
    return false;
  }

  return Math.abs(outputWidth.value / outputHeight.value - 16 / 9) < 0.002;
});
const isNative4k = computed(
  () => outputWidth.value === 3840 && outputHeight.value === 2160,
);
const status = computed(() => {
  if (isNative4k.value) {
    return "슬라이드가 4K UHD와 일치합니다";
  }
  if (isSixteenByNine.value) {
    return "슬라이드는 16:9입니다. 녹화 영역을 바깥 실선에 맞추세요";
  }
  return "슬라이드 비율을 확인하세요";
});

function measureSlide() {
  const slide = guideElement.value?.closest<HTMLElement>(
    ".slidev-slide-content",
  );
  if (!slide) {
    return;
  }

  const rect = slide.getBoundingClientRect();
  const dpr = window.devicePixelRatio || 1;

  devicePixelRatio.value = dpr;

  const edgeWidth =
    Math.round(rect.right * dpr) - Math.round(rect.left * dpr);
  const edgeHeight =
    Math.round(rect.bottom * dpr) - Math.round(rect.top * dpr);
  const sizeWidth = Math.round(rect.width * dpr);
  const sizeHeight = Math.round(rect.height * dpr);

  // Slidev가 가운데 정렬한 슬라이드는 양쪽 경계가 소수점에 놓일 수 있다.
  // 이때 같은 1920×1080 영역도 경계 반올림 방식에 따라 3839px로 보인다.
  // 경계 또는 크기 계산이 4K에서 1px 이내라면 의도한 4K 크기로 맞춘다.
  const isNear4k = [
    [edgeWidth, edgeHeight],
    [sizeWidth, sizeHeight],
  ].some(
    ([width, height]) =>
      Math.abs(width - 3840) <= 1 && Math.abs(height - 2160) <= 1,
  );

  outputWidth.value = isNear4k ? 3840 : edgeWidth;
  outputHeight.value = isNear4k ? 2160 : edgeHeight;
  slideWidth.value = Math.round(outputWidth.value / dpr);
  slideHeight.value = Math.round(outputHeight.value / dpr);
}

function scheduleMeasurement() {
  window.cancelAnimationFrame(animationFrame);
  animationFrame = window.requestAnimationFrame(measureSlide);
}

function updateGuideState() {
  enabled.value =
    new URLSearchParams(window.location.search).get("guide") === "4k";
  void nextTick(() => {
    measureSlide();

    resizeObserver?.disconnect();
    const container = guideElement.value?.closest(".slidev-slide-container");
    if (container) {
      resizeObserver = new ResizeObserver(scheduleMeasurement);
      resizeObserver.observe(container);
    }
  });
}

onMounted(() => {
  updateGuideState();
  window.addEventListener("resize", scheduleMeasurement);
  window.addEventListener("popstate", updateGuideState);
  window.addEventListener("fullscreenchange", scheduleMeasurement);
});

onBeforeUnmount(() => {
  window.cancelAnimationFrame(animationFrame);
  resizeObserver?.disconnect();
  window.removeEventListener("resize", scheduleMeasurement);
  window.removeEventListener("popstate", updateGuideState);
  window.removeEventListener("fullscreenchange", scheduleMeasurement);
});
</script>

<template>
  <div
    v-if="enabled"
    ref="guideElement"
    class="recording-guide"
    :class="{
      'recording-guide--ready': isNative4k,
      'recording-guide--ratio': isSixteenByNine && !isNative4k,
    }"
    aria-hidden="true"
  >
    <div class="recording-guide__safe-area">
      <span>5% 안전 여백</span>
    </div>

    <div class="recording-guide__readout">
      <strong>4K 녹화 가이드</strong>
      <span>
        슬라이드 {{ slideWidth }}×{{ slideHeight }} CSS px · DPR
        {{ devicePixelRatio }} → 예상 {{ outputWidth }}×{{ outputHeight }} px
      </span>
      <span>
        목표 슬라이드 {{ targetSlideWidth }}×{{ targetSlideHeight }} CSS px ·
        {{ status }}
      </span>
    </div>

    <span class="recording-guide__corner recording-guide__corner--tl" />
    <span class="recording-guide__corner recording-guide__corner--tr" />
    <span class="recording-guide__corner recording-guide__corner--bl" />
    <span class="recording-guide__corner recording-guide__corner--br" />
  </div>
</template>

<style>
.recording-guide {
  --recording-guide-color: #ff5c5c;
  border: 4px solid var(--recording-guide-color);
  box-shadow:
    inset 0 0 0 1px rgb(0 0 0 / 75%),
    0 0 0 1px rgb(0 0 0 / 75%);
  box-sizing: border-box;
  inset: 0;
  pointer-events: none;
  position: absolute;
  z-index: 99999;
}

.recording-guide--ratio {
  --recording-guide-color: #f4c95d;
}

.recording-guide--ready {
  --recording-guide-color: #55d6c2;
}

.recording-guide__safe-area {
  border: 2px dashed color-mix(in srgb, var(--recording-guide-color) 70%, transparent);
  inset: 5%;
  position: absolute;
}

.recording-guide__safe-area span {
  background: rgb(10 15 20 / 86%);
  color: var(--recording-guide-color);
  font:
    700 15px/1.2 "SFMono-Regular",
    "Cascadia Code",
    Menlo,
    monospace;
  left: 12px;
  padding: 5px 8px;
  position: absolute;
  top: -14px;
}

.recording-guide__readout {
  backdrop-filter: blur(12px);
  background: rgb(10 15 20 / 90%);
  border: 1px solid var(--recording-guide-color);
  border-radius: 10px;
  bottom: 24px;
  color: #eef3f7;
  display: flex;
  flex-direction: column;
  font:
    600 16px/1.4 "SFMono-Regular",
    "Cascadia Code",
    Menlo,
    monospace;
  gap: 3px;
  left: 50%;
  min-width: 620px;
  padding: 12px 18px;
  position: absolute;
  text-align: center;
  transform: translateX(-50%);
}

.recording-guide__readout strong {
  color: var(--recording-guide-color);
  font-size: 18px;
}

.recording-guide__corner {
  border-color: var(--recording-guide-color);
  height: 32px;
  position: absolute;
  width: 32px;
}

.recording-guide__corner--tl {
  border-left: 8px solid;
  border-top: 8px solid;
  left: 14px;
  top: 14px;
}

.recording-guide__corner--tr {
  border-right: 8px solid;
  border-top: 8px solid;
  right: 14px;
  top: 14px;
}

.recording-guide__corner--bl {
  border-bottom: 8px solid;
  border-left: 8px solid;
  bottom: 14px;
  left: 14px;
}

.recording-guide__corner--br {
  border-bottom: 8px solid;
  border-right: 8px solid;
  bottom: 14px;
  right: 14px;
}
</style>
