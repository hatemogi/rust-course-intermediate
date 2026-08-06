import { readdir, readFile } from "node:fs/promises";
import { relative } from "node:path";

const bookRoot = new URL("../book/src/", import.meta.url);
const episodesRoot = new URL("./episodes/", import.meta.url);

async function markdownFiles(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const nested = await Promise.all(
    entries.map(async (entry) => {
      const path = new URL(`${entry.name}${entry.isDirectory() ? "/" : ""}`, directory);
      if (entry.isDirectory()) return markdownFiles(path);
      return entry.name.endsWith(".md") ? [path] : [];
    }),
  );
  return nested.flat();
}

function plainHeading(markdown) {
  return markdown
    .replace(/`([^`]+)`/g, "$1")
    .replace(/<[^>]+>/g, "")
    .replace(/\s*◆\s*$/, "")
    .trim();
}

const episodeFiles = (await markdownFiles(episodesRoot)).filter(
  (file) => !file.pathname.endsWith("/all.md"),
);
const notes = (
  await Promise.all(episodeFiles.map((file) => readFile(file, "utf8")))
).join("\n");

const missing = [];
for (const file of await markdownFiles(bookRoot)) {
  if (file.pathname.endsWith("/SUMMARY.md")) continue;

  const markdown = await readFile(file, "utf8");
  for (const match of markdown.matchAll(/^#{1,3}\s+(.+)$/gm)) {
    const heading = plainHeading(match[1]);
    if (!notes.includes(`> ${heading}`)) {
      missing.push(`${relative(bookRoot.pathname, file.pathname)} > ${heading}`);
    }
  }
}

if (missing.length > 0) {
  console.error("발표자 노트에서 다음 mdBook 제목의 대응을 찾지 못했습니다:");
  for (const heading of missing) console.error(`- ${heading}`);
  process.exitCode = 1;
} else {
  console.log("발표자 노트가 mdBook의 모든 1~3단계 제목에 대응합니다.");
}
