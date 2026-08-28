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
    { appId: "2397320", name: "Bus Simulator 27" },
    { appId: "275850", name: "No Man's Sky" },
    { appId: "12345", name: "No Man's Sky 2" },
  ];

  invoke = async (cmd, args) => {
    await wait(400);
    if (cmd === "fetch_steam_trailers") {
      if (!/^\d+$/.test(args.appId)) throw "Steam returned no data for that App ID.";
      return { gameName: "Parcel Simulator", trailers: MOCK_TRAILERS };
    }
    if (cmd === "search_steam_games_by_name") {
      const q = (args.query || "").trim().toLowerCase();
      if (!q) return [];
      return MOCK_SEARCH_RESULTS.filter((item) => item.name.toLowerCase().includes(q));
    }
    if (cmd === "get_default_download_dir") return "C:\\Users\\barry\\Downloads";
    if (cmd === "pick_folder") return "G:\\1_Presskit";
    if (cmd === "find_ffmpeg") return { path: "F:\\ffmpeg\\bin\\ffmpeg.exe", version: "N-120041-g64fce7202c-20250626" };
    if (cmd === "load_history") return [];
    if (cmd === "save_history") return null;
    if (cmd === "set_ffmpeg_path") return { path: args.path, version: "N-120041-g64fce7202c-20250626" };
    if (cmd === "install_ffmpeg") return { path: "C:\\Users\\barry\\AppData\\Local\\com.ackrosgaming.gcc\\tools\\ffmpeg\\bin\\ffmpeg.exe", version: "N-120041-g64fce7202c-20250626" };
    if (cmd === "get_app_metadata") {
      return { releaseType: "ALPHA", version: "0.1.4", build: "preview", creator: "AckrosGaming" };
    }
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

function activateTab(tabName) {
  const tab = document.querySelector(`.rail-tab[data-tab="${tabName}"]`);
  const pane = document.getElementById(`pane-${tabName}`);
  if (!tab || !pane) return;

  document.querySelectorAll(".rail-tab").forEach((item) => item.classList.remove("is-active"));
  document.querySelectorAll(".pane").forEach((item) => item.classList.remove("is-active"));
  tab.classList.add("is-active");
  pane.classList.add("is-active");
}

// ── Themes ──────────────────────────────────────────────────────────────

const THEME_DEFAULT_KEY = "ggt-default-theme";
const THEME_CUSTOM_KEY = "ggt-theme-customizations";
const THEME_LIBRARY_KEY = "ggt-theme-library";
const THEME_NAMES_KEY = "ggt-theme-names";
const THEME_DELETED_KEY = "ggt-deleted-themes";
const BUILT_IN_THEMES = {
  default: {
    name: "Default",
    colors: {
      "bg-app": "#0b0e14",
      "bg-panel": "#11151d",
      "bg-panel-2": "#171c26",
      "bg-hover": "#1c2230",
      border: "#232a37",
      "border-strong": "#313a4b",
      "text-primary": "#e7edf5",
      "text-secondary": "#8b96a9",
      "text-muted": "#5a6478",
      cyan: "#4fd1c9",
      "cyan-dim": "#2c6863",
      amber: "#e8b339",
    },
  },
  "navy-white-1": {
    name: "NavyWhite1",
    colors: {
      "bg-app": "#ffffff",
      "bg-panel": "#f7f9fc",
      "bg-panel-2": "#edf2f7",
      "bg-hover": "#e3eaf2",
      border: "#d7dee8",
      "border-strong": "#b6c2d1",
      "text-primary": "#142033",
      "text-secondary": "#43536a",
      "text-muted": "#68778b",
      cyan: "#147d89",
      "cyan-dim": "#236976",
      amber: "#9a6700",
    },
  },
};

const themePicker = document.getElementById("theme-picker");
const themeDefaultToggle = document.getElementById("theme-default-toggle");
const themeDefaultStatus = document.getElementById("theme-default-status");
const themeReset = document.getElementById("theme-reset");
const themeEditorStatus = document.getElementById("theme-editor-status");
const themeNameInput = document.getElementById("theme-name");
const themeUpdate = document.getElementById("theme-update");
const themeSaveAs = document.getElementById("theme-save-as");
const themeRename = document.getElementById("theme-rename");
const themeDelete = document.getElementById("theme-delete");
const themeColorInputs = [...document.querySelectorAll("[data-theme-color]")];

function readStoredObject(key) {
  try {
    const stored = JSON.parse(localStorage.getItem(key) || "{}");
    return stored && typeof stored === "object" ? stored : {};
  } catch {
    return {};
  }
}

let themeOverrides = readStoredObject(THEME_CUSTOM_KEY);
let customThemes = readStoredObject(THEME_LIBRARY_KEY);
let themeNames = readStoredObject(THEME_NAMES_KEY);
let deletedThemeIds = new Set(Object.keys(readStoredObject(THEME_DELETED_KEY)));
let defaultThemeId = localStorage.getItem(THEME_DEFAULT_KEY);
let draftColors = {};

function getTheme(themeId) {
  if (customThemes[themeId]) return customThemes[themeId];
  if (BUILT_IN_THEMES[themeId] && !deletedThemeIds.has(themeId)) return BUILT_IN_THEMES[themeId];
  return null;
}

if (!getTheme(defaultThemeId)) defaultThemeId = "default";
let activeThemeId = defaultThemeId;

function getThemeName(themeId) {
  if (themeId === "default") return "Default";
  return themeNames[themeId] || getTheme(themeId)?.name || "Untitled theme";
}

function getSavedThemeColors(themeId) {
  const theme = getTheme(themeId);
  if (!theme) return { ...BUILT_IN_THEMES.default.colors };
  return customThemes[themeId]
    ? { ...theme.colors }
    : { ...theme.colors, ...(themeOverrides[themeId] || {}) };
}

function persistThemeLibrary() {
  localStorage.setItem(THEME_CUSTOM_KEY, JSON.stringify(themeOverrides));
  localStorage.setItem(THEME_LIBRARY_KEY, JSON.stringify(customThemes));
  localStorage.setItem(THEME_NAMES_KEY, JSON.stringify(themeNames));
  localStorage.setItem(
    THEME_DELETED_KEY,
    JSON.stringify(Object.fromEntries([...deletedThemeIds].map((themeId) => [themeId, true])))
  );
}

function populateThemePicker() {
  themePicker.innerHTML = "";
  const themeIds = [
    ...Object.keys(BUILT_IN_THEMES).filter((themeId) => !deletedThemeIds.has(themeId)),
    ...Object.keys(customThemes),
  ];
  themeIds.forEach((themeId) => {
    const option = document.createElement("option");
    option.value = themeId;
    option.textContent = getThemeName(themeId);
    themePicker.appendChild(option);
  });
}

function setThemeEditorStatus(message, isError = false) {
  themeEditorStatus.textContent = message;
  themeEditorStatus.classList.toggle("is-error", isError);
}

function updateThemeControls() {
  populateThemePicker();
  themePicker.value = activeThemeId;
  themeDefaultToggle.checked = activeThemeId === defaultThemeId;
  themeDefaultToggle.disabled = activeThemeId === "default" && defaultThemeId === "default";
  themeDefaultStatus.textContent = `${getThemeName(defaultThemeId)} is the startup theme`;
  themeNameInput.value = getThemeName(activeThemeId);
  themeRename.disabled = activeThemeId === "default";
  themeDelete.disabled = activeThemeId === "default";
  themeColorInputs.forEach((input) => {
    input.value = draftColors[input.dataset.themeColor];
  });
}

function applyTheme(themeId) {
  activeThemeId = getTheme(themeId) ? themeId : "default";
  draftColors = getSavedThemeColors(activeThemeId);
  Object.entries(draftColors).forEach(([name, value]) => {
    document.documentElement.style.setProperty(`--${name}`, value);
  });
  document.documentElement.dataset.theme = activeThemeId;
  setThemeEditorStatus("Changes preview immediately");
  updateThemeControls();
}

themePicker.addEventListener("change", () => applyTheme(themePicker.value));

themeDefaultToggle.addEventListener("change", () => {
  defaultThemeId = themeDefaultToggle.checked ? activeThemeId : "default";
  localStorage.setItem(THEME_DEFAULT_KEY, defaultThemeId);
  updateThemeControls();
});

themeColorInputs.forEach((input) => {
  input.addEventListener("input", () => {
    const colorName = input.dataset.themeColor;
    draftColors[colorName] = input.value;
    document.documentElement.style.setProperty(`--${colorName}`, input.value);
    setThemeEditorStatus("Unsaved color changes");
  });
});

themeReset.addEventListener("click", () => {
  draftColors = customThemes[activeThemeId]
    ? getSavedThemeColors(activeThemeId)
    : { ...BUILT_IN_THEMES[activeThemeId].colors };
  Object.entries(draftColors).forEach(([name, value]) => {
    document.documentElement.style.setProperty(`--${name}`, value);
  });
  setThemeEditorStatus(customThemes[activeThemeId] ? "Reverted to saved colors" : "Factory colors ready to update");
  updateThemeControls();
});

themeUpdate.addEventListener("click", () => {
  if (customThemes[activeThemeId]) {
    customThemes[activeThemeId].colors = { ...draftColors };
  } else {
    themeOverrides[activeThemeId] = { ...draftColors };
  }
  persistThemeLibrary();
  setThemeEditorStatus(`${getThemeName(activeThemeId)} updated`);
});

function validatedThemeName(allowActiveTheme) {
  const name = themeNameInput.value.trim();
  if (!name) {
    setThemeEditorStatus("Enter a theme name", true);
    themeNameInput.focus();
    return null;
  }
  const duplicate = [...themePicker.options].some(
    (option) =>
      (!allowActiveTheme || option.value !== activeThemeId) &&
      option.textContent.toLowerCase() === name.toLowerCase()
  );
  if (duplicate) {
    setThemeEditorStatus("A theme with that name already exists", true);
    themeNameInput.focus();
    return null;
  }
  return name;
}

themeSaveAs.addEventListener("click", () => {
  const name = validatedThemeName(false);
  if (!name) return;
  const themeId = `custom-${Date.now()}`;
  customThemes[themeId] = { name, colors: { ...draftColors } };
  persistThemeLibrary();
  applyTheme(themeId);
  setThemeEditorStatus(`${name} saved as a new theme`);
});

themeRename.addEventListener("click", () => {
  if (activeThemeId === "default") return;
  const name = validatedThemeName(true);
  if (!name) return;
  themeNames[activeThemeId] = name;
  if (customThemes[activeThemeId]) customThemes[activeThemeId].name = name;
  persistThemeLibrary();
  updateThemeControls();
  setThemeEditorStatus(`Theme renamed to ${name}`);
});

themeDelete.addEventListener("click", () => {
  if (activeThemeId === "default") return;
  const deletedId = activeThemeId;
  const deletedName = getThemeName(deletedId);
  if (!window.confirm(`Delete theme "${deletedName}"?`)) return;
  delete customThemes[deletedId];
  delete themeOverrides[deletedId];
  delete themeNames[deletedId];
  if (BUILT_IN_THEMES[deletedId]) deletedThemeIds.add(deletedId);
  if (defaultThemeId === deletedId) {
    defaultThemeId = "default";
    localStorage.setItem(THEME_DEFAULT_KEY, defaultThemeId);
  }
  persistThemeLibrary();
  applyTheme("default");
  setThemeEditorStatus(`${deletedName} deleted`);
});

applyTheme(activeThemeId);

document.querySelectorAll(".rail-tab").forEach((tab) => {
  tab.addEventListener("click", () => activateTab(tab.dataset.tab));
});

document.querySelectorAll("[data-open-tab]").forEach((control) => {
  control.addEventListener("click", () => activateTab(control.dataset.openTab));
});

invoke("get_app_metadata").then((metadata) => {
  const release = `${metadata.releaseType} ${metadata.version}.${metadata.build}`;
  document.getElementById("home-release-type").textContent = metadata.releaseType;
  document.getElementById("home-release-version").textContent = `${metadata.version}.${metadata.build}`;
  document.getElementById("home-creator").textContent = metadata.creator;
  document.getElementById("settings-release").textContent = release;
  document.getElementById("settings-creator").textContent = metadata.creator;
});

// ── Steam tab ────────────────────────────────────────────────────────────

const steamInput = document.getElementById("steam-input");
const steamPasteBtn = document.getElementById("steam-paste");
const steamClearBtn = document.getElementById("steam-clear");
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
let currentGameName = null;
let currentTrailers = [];
let outputDir = null;
let currentDownloadName = null;

// ── History (backend JSON, with one-time localStorage migration) ───────────

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
let historySave = Promise.resolve();

async function hydrateHistory() {
  if (!hasTauri) {
    updateActivity();
    return;
  }
  try {
    const durableHistory = await invoke("load_history");
    const merged = new Map();
    [...durableHistory, ...history].forEach((entry) => {
      merged.set(JSON.stringify(entry), entry);
    });
    history = [...merged.values()]
      .sort((left, right) => String(left.ts).localeCompare(String(right.ts)))
      .slice(-HISTORY_LIMIT);
    await invoke("save_history", { history });
    localStorage.removeItem(HISTORY_KEY);
  } catch (error) {
    console.error("Could not load durable history:", error);
  }
  updateActivity();
}

function saveHistory() {
  if (hasTauri) {
    const snapshot = [...history];
    historySave = historySave
      .then(() => invoke("save_history", { history: snapshot }))
      .catch((error) => {
        console.error("Could not save durable history:", error);
      });
  } else {
    localStorage.setItem(HISTORY_KEY, JSON.stringify(history));
  }
}

function pushHistory(entry) {
  history.push({ ts: new Date().toISOString(), ...entry });
  if (history.length > HISTORY_LIMIT) history = history.slice(-HISTORY_LIMIT);
  saveHistory();
  updateActivity();
}

function updateActivity() {
  const activity = document.getElementById("settings-activity-count");
  if (!history.length) {
    activity.textContent = "No activity yet";
    return;
  }

  const last = new Date(history[history.length - 1].ts).toLocaleDateString();
  activity.textContent = `${history.length} event${history.length === 1 ? "" : "s"} / last ${last}`;
}
hydrateHistory();

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
  updateActivity();
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

let ffmpegInfo = null;

function updateFfmpegStatus(info) {
  ffmpegInfo = info;
  const el = document.getElementById("settings-ffmpeg-path");
  el.textContent = info?.path || "Not found";
  document.getElementById("settings-ffmpeg-version").innerHTML = info
    ? `<span class="status-dot is-ready"></span>ffmpeg ${escapeHtml(info.version)}`
    : '<span class="status-dot is-warning"></span>ffmpeg required';
  document.getElementById("steam-ffmpeg-warning").hidden = Boolean(info);
}

invoke("find_ffmpeg").then(updateFfmpegStatus);

async function installFfmpeg() {
  const buttons = [...document.querySelectorAll(".install-ffmpeg")];
  buttons.forEach((button) => {
    button.disabled = true;
    button.textContent = "Downloading and verifying...";
  });
  try {
    const path = await invoke("install_ffmpeg");
    updateFfmpegStatus(path);
  } catch (error) {
    updateFfmpegStatus(ffmpegInfo);
    document.getElementById("settings-ffmpeg-path").textContent = String(error);
  } finally {
    buttons.forEach((button) => {
      button.disabled = false;
      button.textContent = button.closest(".dependency-warning") ? "Install ffmpeg" : "Install";
    });
  }
}

document.querySelectorAll(".install-ffmpeg").forEach((button) => {
  button.addEventListener("click", installFfmpeg);
});

document.getElementById("settings-change-ffmpeg").addEventListener("click", async () => {
  const picked = hasTauri
    ? await window.__TAURI__.dialog.open({
        multiple: false,
        filters: [{ name: "ffmpeg executable", extensions: ["exe"] }],
      })
    : "F:\\ffmpeg\\bin\\ffmpeg.exe";
  if (!picked) return;
  try {
    updateFfmpegStatus(await invoke("set_ffmpeg_path", { path: picked }));
  } catch (error) {
    document.getElementById("settings-ffmpeg-path").textContent = String(error);
  }
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
steamInput.addEventListener("input", updateSteamInputActions);
steamPasteBtn.addEventListener("click", async () => {
  try {
    const text = hasTauri
      ? await window.__TAURI__.clipboardManager.readText()
      : await navigator.clipboard.readText();
    if (!text?.trim()) throw new Error("The clipboard does not contain text.");
    steamInput.value = text.trim();
    updateSteamInputActions();
    steamInput.focus();
    steamStatus.textContent = "Pasted from clipboard. Click Find trailers when ready.";
    steamStatus.className = "status-line is-success";
  } catch (error) {
    steamStatus.textContent = `Could not paste: ${error}`;
    steamStatus.className = "status-line is-error";
  }
});

steamClearBtn.addEventListener("click", () => {
  steamInput.value = "";
  steamNameResults.hidden = true;
  steamStatus.textContent = "";
  steamStatus.className = "status-line";
  updateSteamInputActions();
  steamInput.focus();
});

function updateSteamInputActions() {
  steamClearBtn.disabled = !steamInput.value;
}

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
      updateSteamInputActions();
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
    currentGameName = data.gameName;
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
      gameName: currentGameName,
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
