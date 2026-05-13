// Generates assets/icon.png (1024x1024) from assets/icon.svg using @resvg/resvg-js.
// Then `npm run tauri icon` is expected to convert it to the platform icon set.
import { readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { Resvg } from "@resvg/resvg-js";

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = join(__dirname, "..");

const svg = await readFile(join(root, "assets", "icon.svg"), "utf-8");
const resvg = new Resvg(svg, {
  fitTo: { mode: "width", value: 1024 },
  background: "rgba(0,0,0,0)",
});
const png = resvg.render().asPng();
await writeFile(join(root, "assets", "icon.png"), png);
console.log(`Wrote assets/icon.png (${png.length} bytes)`);
