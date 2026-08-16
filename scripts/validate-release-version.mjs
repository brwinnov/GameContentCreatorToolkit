import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const appDirectory = path.resolve(scriptDirectory, "..", "app");

const packageVersion = JSON.parse(
  fs.readFileSync(path.join(appDirectory, "package.json"), "utf8"),
).version;
const lockVersion = JSON.parse(
  fs.readFileSync(path.join(appDirectory, "package-lock.json"), "utf8"),
).version;
const tauriVersion = JSON.parse(
  fs.readFileSync(path.join(appDirectory, "src-tauri", "tauri.conf.json"), "utf8"),
).version;
const cargoManifest = fs.readFileSync(
  path.join(appDirectory, "src-tauri", "Cargo.toml"),
  "utf8",
);
const cargoVersion = cargoManifest.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
const tagVersion = process.env.GITHUB_REF_NAME?.replace(/^v/, "");

const versions = {
  tag: tagVersion,
  npm: packageVersion,
  lock: lockVersion,
  tauri: tauriVersion,
  cargo: cargoVersion,
};

console.log("Release versions:", versions);

if (!tagVersion || Object.values(versions).some((version) => version !== tagVersion)) {
  console.error("Release tag, npm, lockfile, Cargo, and Tauri versions must match.");
  process.exit(1);
}