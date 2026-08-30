const recordingParameters = new URLSearchParams(window.location.search);
const pipGuide = recordingParameters.get("guide") === "pip";

document.documentElement.classList.toggle("recording-pip-guide", pipGuide);

if (pipGuide) {
    for (const link of document.querySelectorAll("a[href]")) {
        const rawHref = link.getAttribute("href");
        if (!rawHref || rawHref.startsWith("#")) continue;

        const target = new URL(rawHref, window.location.href);
        if (target.origin !== window.location.origin) continue;

        target.searchParams.set("guide", "pip");
        link.href = target.href;
    }
}
