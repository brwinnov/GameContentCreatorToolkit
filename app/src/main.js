// ── Tauri bridge (falls back to a mock so this file also works when ─────────
// opened directly in a browser, e.g. by double-clicking index.html) ────────

const hasTauri = typeof window.__TAURI__ !== "undefined";

let invoke, listen;

if (hasTauri) {
  invoke = window.__TAURI__.core.invoke;
  listen = window.__TAURI__.event.listen;
} else {
  console.warn("[preview mode] Tauri APIs not found — using mock data. Open this inside the built app for real downloads.");

  const MOCK_TRAILERS = [
    { name: "Parcel Simulator - New Warehouse Trailer", dashUrl: "mock://1" },
    { name: "Parcel Simulator Trailer", dashUrl: "mock://2" },
  ];

  const MOCK_SEARCH_RESULTS = [
    { appId: "275850", name: "No Man's Sky" },
    { appId: "12345", name: "No Man's Sky 2" },
  ];

  invoke = async (cmd, args) => {
    await wait(400);
    if (cmd === "fetch_steam_trailers") {
      if (!/^\d+$/.test(args.appId)) throw "Steam returned no data for that App ID.";
      return { trailers: MOCK_TRAILERS };
    }
    if (cmd === "search_steam_games_by_name") {
      const q = (args.query || "").trim().toLowerCase();
      if (!q) return [];
      return MOCK_SEARCH_RESULTS.filter((item) => item.name.toLowerCase().includes(q));
    }
    if (cmd === "get_default_download_dir") return "C:\\Users\\barry\\Downloads";
    if (cmd === "pick_folder") return "G:\\1_Presskit";
    if (cmd === "find_ffmpeg") return "F:\\ffmpeg\\bin\\ffmpeg.exe";
    if (cmd === "download_trailers") {
      const emit = window.__mockEmit;
      for (const t of args.trailers) {
        emit("download-progress", { tag: "DOWN", message: t.name, percent: 0 });
        for (let p = 15; p < 100; p += 20) {
          await wait(150);
          emit("download-progress", { tag: "PROGRESS", message: t.name, percent: p });
        }
        await wait(150);
        emit("download-progress", { tag: "OK", message: `Saved -> ${args.outputDir}\\${slug(t.name)}.mp4  (17.3 MB)`, percent: 100 });
        await wait(200);
      }
      return { success: args.trailers.length, skipped: 0, failed: 0 };
    }
    return null;
  };

  const listeners = {};
  window.__mockEmit = (event, payload) => (listeners[event] || []).forEach((cb) => cb({ payload }));
  listen = async (event, cb) => {
    listeners[event] = listeners[event] || [];
    listeners[event].push(cb);
    return () => {};
  };

  function wait(ms) { return new Promise((r) => setTimeout(r, ms)); }
  function slug(name) {
    return name.toLowerCase().replace(/[^\w\s-]/g, "").replace(/\s+/g, "_").replace(/_+/g, "_").replace(/^_|_$/g, "");
  }
}

// ── Tab navigation ───────────────────────────────────────────────────────

document.querySelectorAll(".rail-tab").forEach((tab) => {
  tab.addEventListener("click", () => {
    document.querySelectorAll(".rail-tab").forEach((t) => t.classList.remove("is-active"));
    document.querySelectorAll(".pane").forEach((p) => p.classList.remove("is-active"));
    tab.classList.add("is-active");
    document.getElementById(`pane-${tab.dataset.tab}`).classList.add("is-active");
  });
});

// ── Steam tab ────────────────────────────────────────────────────────────

const steamInput = document.getElementById("steam-input");
const steamFetchBtn = document.getElementById("steam-fetch");
const steamNameResults = document.getElementById("steam-name-results");
const steamStatus = document.getElementById("steam-status");
const steamResults = document.getElementById("steam-results");
const steamList = document.getElementById("steam-list");
const steamCount = document.getElementById("steam-count");
const steamFooter = document.getElementById("steam-footer");
const steamOutputPath = document.getElementById("steam-output-path");
const steamChangeDir = document.getElementById("steam-change-dir");
const steamDownloadBtn = document.getElementById("steam-download");
const steamConsole = document.getElementById("steam-console");
const steamConsoleBody = document.getElementById("steam-console-body");
const steamConsoleClear = document.getElementById("steam-console-clear");
const steamProgressRow = document.getElementById("steam-progress-row");
const steamProgressFill = document.getElementById("steam-progress-fill");
const steamProgressPct = document.getElementById("steam-progress-pct");
const steamHistoryBtn = document.getElementById("steam-history");
const historyModal = document.getElementById("history-modal");
const historyBody = document.getElementById("history-body");
const historyClose = document.getElementById("history-close");
const historyClearBtn = document.getElementById("history-clear");

let currentAppId = null;
let currentTrailers = [];
let outputDir = null;
let currentDownloadName = null;

// ── History (persisted in localStorage — survives app restarts) ────────────

const HISTORY_KEY = "ggt-steam-history";
const HISTORY_LIMIT = 200; // keep the log from growing unbounded

function loadHistory() {
  try {
    const raw = localStorage.getItem(HISTORY_KEY);
    return raw ? JSON.parse(raw) : [];
  } catch {
    return [];
  }
}

let history = loadHistory();

function saveHistory() {
  localStorage.setItem(HISTORY_KEY, JSON.stringify(history));
}

function pushHistory(entry) {
  history.push({ ts: new Date().toISOString(), ...entry });
  if (history.length > HISTORY_LIMIT) history = history.slice(-HISTORY_LIMIT);
  saveHistory();
}

function renderHistory() {
  if (!history.length) {
    historyBody.innerHTML = '<div class="history-empty">No history yet.</div>';
    return;
  }

  historyBody.innerHTML = [...history]
    .reverse()
    .map((e) => {
      const time = new Date(e.ts).toLocaleString();
      if (e.kind === "search") {
        const badge = e.status === "invalid" ? "INVALID" : "SEARCH";
        const idPart = e.appId ? ` (${e.appId})` : "";
        return `<div class="history-row">
          <span class="history-time">${time}</span>
          <span class="history-tag tag-${badge}">${badge}</span>
          <span class="history-msg">"${escapeHtml(e.input)}"${idPart} — ${escapeHtml(e.detail)}</span>
        </div>`;
      }
      const badge = e.status.toUpperCase();
      return `<div class="history-row">
        <span class="history-time">${time}</span>
        <span class="history-tag tag-${badge}">${badge}</span>
        <span class="history-msg">[${e.appId ?? "—"}] ${escapeHtml(e.name)}</span>
      </div>`;
    })
    .join("");
}

steamHistoryBtn.addEventListener("click", () => {
  renderHistory();
  historyModal.hidden = false;
});
historyClose.addEventListener("click", () => (historyModal.hidden = true));
historyModal.addEventListener("click", (e) => {
  if (e.target === historyModal) historyModal.hidden = true;
});
document.addEventListener("keydown", (e) => {
  if (e.key === "Escape" && !historyModal.hidden) historyModal.hidden = true;
});
historyClearBtn.addEventListener("click", () => {
  history = [];
  saveHistory();
  renderHistory();
});

function extractAppId(raw) {
  const s = raw.trim();
  const urlMatch = s.match(/\/app\/(\d+)/);
  if (urlMatch) return urlMatch[1];
  if (/^\d+$/.test(s)) return s;
  return null;
}

async function initOutputDir() {
  outputDir = await invoke("get_default_download_dir");
  steamOutputPath.textContent = outputDir;
  document.getElementById("settings-output-path").textContent = outputDir;
}
initOutputDir();

invoke("find_ffmpeg").then((path) => {
  const el = document.getElementById("settings-ffmpeg-path");
  el.textContent = path || "Not found — install ffmpeg and restart the app";
});

steamChangeDir.addEventListener("click", async () => {
  const picked = hasTauri
    ? await window.__TAURI__.dialog.open({ directory: true, multiple: false })
    : await invoke("pick_folder"); // mock fallback for browser preview
  if (picked) {
    outputDir = picked;
    steamOutputPath.textContent = outputDir;
    document.getElementById("settings-output-path").textContent = outputDir;
  }
});
document.getElementById("settings-change-dir").addEventListener("click", () => steamChangeDir.click());

steamFetchBtn.addEventListener("click", fetchTrailers);
steamInput.addEventListener("keydown", (e) => { if (e.key === "Enter") fetchTrailers(); });

function renderSteamNameMatches(matches) {
  steamNameResults.hidden = false;
  steamNameResults.innerHTML = "";

  const title = document.createElement("div");
  title.className = "search-results-title";
  title.textContent = "Choose a matching game:";
  steamNameResults.appendChild(title);

  matches.forEach((match) => {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "search-result-item";
    btn.textContent = `${match.name} (${match.appId})`;
    btn.addEventListener("click", () => {
      steamInput.value = match.appId;
      steamNameResults.hidden = true;
      fetchTrailers();
    });
    steamNameResults.appendChild(btn);
  });
}

async function fetchTrailers() {
  const rawInput = steamInput.value.trim();
  const appId = extractAppId(rawInput);

  if (!appId) {
    if (!rawInput) {
      steamStatus.textContent = "Enter a Steam store URL, App ID, or game name.";
      steamStatus.className = "status-line is-error";
      steamNameResults.hidden = true;
      pushHistory({
        kind: "search",
        input: rawInput,
        appId: null,
        status: "invalid",
        detail: "Enter a Steam store URL, App ID, or game name.",
      });
      return;
    }

    steamStatus.textContent = `Searching Steam for “${rawInput}”…`;
    steamStatus.className = "status-line";
    steamResults.hidden = true;
    steamFooter.hidden = true;
    steamFetchBtn.disabled = true;

    try {
      const matches = await invoke("search_steam_games_by_name", { query: rawInput });
      if (!matches.length) {
        steamNameResults.hidden = true;
        steamStatus.textContent = `No Steam game matches found for “${rawInput}”.`;
        steamStatus.className = "status-line is-error";
        pushHistory({ kind: "search", input: rawInput, appId: null, status: "invalid", detail: "No matching game found." });
        return;
      }

      renderSteamNameMatches(matches);
      steamStatus.textContent = `Found ${matches.length} matches. Pick the correct game to continue.`;
      steamStatus.className = "status-line";
      pushHistory({ kind: "search", input: rawInput, appId: null, status: "ok", detail: `Found ${matches.length} game matches.` });
      return;
    } catch (err) {
      steamNameResults.hidden = true;
      steamStatus.textContent = String(err);
      steamStatus.className = "status-line is-error";
      pushHistory({ kind: "search", input: rawInput, appId: null, status: "invalid", detail: String(err) });
      return;
    } finally {
      steamFetchBtn.disabled = false;
    }
  }

  currentAppId = appId;
  steamStatus.textContent = `Querying Steam API for App ID ${appId}…`;
  steamStatus.className = "status-line";
  steamResults.hidden = true;
  steamFooter.hidden = true;
  steamFetchBtn.disabled = true;
  steamNameResults.hidden = true;

  try {
    const data = await invoke("fetch_steam_trailers", { appId });
    currentTrailers = data.trailers;

    if (!currentTrailers.length) {
      steamStatus.textContent = `No trailers found for App ID ${appId}.`;
      steamStatus.className = "status-line is-error";
      pushHistory({ kind: "search", input: rawInput, appId, status: "invalid", detail: "No trailers found." });
      return;
    }

    steamStatus.textContent = `Found ${currentTrailers.length} trailer(s) — newest first.`;
    steamStatus.className = "status-line is-success";
    pushHistory({
      kind: "search",
      input: rawInput,
      appId,
      status: "ok",
      detail: `Found ${currentTrailers.length} trailer(s).`,
    });
    renderTrailerList();
    steamResults.hidden = false;
    steamFooter.hidden = false;
  } catch (err) {
    steamStatus.textContent = String(err);
    steamStatus.className = "status-line is-error";
    pushHistory({ kind: "search", input: rawInput, appId, status: "invalid", detail: String(err) });
  } finally {
    steamFetchBtn.disabled = false;
  }
}

function renderTrailerList() {
  steamList.innerHTML = "";
  currentTrailers.forEach((t, i) => {
    const li = document.createElement("li");
    li.className = "trailer-item";
    li.innerHTML = `
      <input type="checkbox" data-idx="${i}" ${i === 0 ? "checked" : ""} />
      <span class="idx">${i + 1}.</span>
      <span class="name">${escapeHtml(t.name)}</span>
      ${i === 0 ? '<span class="badge-newest">newest</span>' : ""}
    `;
    li.addEventListener("click", (e) => {
      if (e.target.tagName !== "INPUT") {
        const cb = li.querySelector("input");
        cb.checked = !cb.checked;
      }
      updateSelectionState();
    });
    steamList.appendChild(li);
  });
  updateSelectionState();
}

function getCheckboxes() { return [...steamList.querySelectorAll("input[type=checkbox]")]; }

function updateSelectionState() {
  const boxes = getCheckboxes();
  const selected = boxes.filter((b) => b.checked).length;
  steamCount.textContent = `${selected} of ${currentTrailers.length} selected`;
  steamDownloadBtn.disabled = selected === 0;
}

document.getElementById("steam-select-all").addEventListener("click", () => {
  getCheckboxes().forEach((b) => (b.checked = true));
  updateSelectionState();
});
document.getElementById("steam-select-latest").addEventListener("click", () => {
  getCheckboxes().forEach((b, i) => (b.checked = i === 0));
  updateSelectionState();
});
document.getElementById("steam-select-none").addEventListener("click", () => {
  getCheckboxes().forEach((b) => (b.checked = false));
  updateSelectionState();
});

steamConsoleClear.addEventListener("click", () => (steamConsoleBody.innerHTML = ""));

function logLine(tag, message) {
  const line = document.createElement("div");
  line.className = `console-line tag-${tag}`;
  line.textContent = `[${tag}] ${message}`;
  steamConsoleBody.appendChild(line);
  steamConsoleBody.scrollTop = steamConsoleBody.scrollHeight;
}

function setProgress(pct) {
  const clamped = Math.max(0, Math.min(100, pct));
  steamProgressRow.hidden = false;
  steamProgressFill.style.width = `${clamped}%`;
  steamProgressPct.textContent = `${clamped}%`;
}

// A trailer with no dash_h264 URL is unusable outright (an "invalid" entry),
// distinct from "skipped" (the file already exists) — both currently arrive
// as SKIP from the backend, so split on the message text instead of adding a
// new backend tag.
function classifyDownloadStatus(tag, message) {
  if (tag === "OK") return "downloaded";
  if (tag === "FAIL") return "failed";
  if (tag === "SKIP") return /dash_h264|url available/i.test(message) ? "invalid" : "skipped";
  return null;
}

listen("download-progress", (event) => {
  const { tag, message, percent } = event.payload;

  if (tag === "PROGRESS") {
    if (percent != null) setProgress(percent);
    return; // per-file progress ticks don't get their own console line
  }

  if (tag === "DOWN") {
    currentDownloadName = message;
    setProgress(percent ?? 0);
  }
  if (tag === "OK") setProgress(percent ?? 100);
  if (tag === "FAIL" || tag === "SKIP") steamProgressRow.hidden = true;

  const status = classifyDownloadStatus(tag, message);
  if (status) {
    // OK's message is the saved file path, not the trailer name — use the
    // name captured from the preceding DOWN event instead.
    const name = tag === "OK" ? currentDownloadName : message.split(" — ")[0];
    pushHistory({ kind: "download", appId: currentAppId, name: name || message, status });
  }

  logLine(tag, message);
});

steamDownloadBtn.addEventListener("click", async () => {
  const selected = getCheckboxes()
    .map((b, i) => (b.checked ? currentTrailers[i] : null))
    .filter(Boolean);

  if (!selected.length) return;

  steamConsole.hidden = false;
  steamConsoleBody.innerHTML = "";
  steamProgressRow.hidden = true;
  steamDownloadBtn.disabled = true;
  logLine("INFO", `Downloading ${selected.length} trailer(s) to ${outputDir}…`);

  try {
    const summary = await invoke("download_trailers", {
      appId: currentAppId,
      outputDir,
      trailers: selected,
    });
    logLine("INFO", `Done — downloaded ${summary.success}, skipped ${summary.skipped}, failed ${summary.failed}.`);
  } catch (err) {
    logLine("FAIL", String(err));
  } finally {
    steamDownloadBtn.disabled = false;
  }
});

function escapeHtml(s) {
  return s.replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c]));
}
