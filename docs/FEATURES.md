# Feature Backlog

Reviewable list of product and engineering ideas for GCCtoolkit. Use this file
to choose what to build next, add new ideas, and record decisions without
turning `TODO.md` into a long-term wishlist.

## How to Use This List

- **Status:** `shipped`, `ready`, `planned`, `idea`, `deferred`, or `rejected`
- **Priority:** `P0` release blocker, `P1` high value, `P2` useful, `P3` later
- **Effort:** rough relative estimate: `S`, `M`, `L`, or `XL`
- Move selected work into `TODO.md` with acceptance criteria.
- Keep architecture details in focused plans such as `repo-update.md`.
- Mark delivered behavior in `CHANGELOG.md`.

## Suggested Next Choices

These are the strongest candidates for the next focused milestone:

| ID | Feature | Why now |
| --- | --- | --- |
| STM-03 | Screenshots and key art | High creator value using data already available from Steam. |
| STM-02 | Metadata sidecars | Small, useful foundation for organisation and Resolve workflows. |
| STM-04 | Batch game input | Turns one-game lookup into a repeatable production workflow. |
| APP-03 | Download queue and batch mode | Turns the app from a one-game tool into a repeatable production utility. |
| DEP-07 | Third-party notices | Required before broader distribution of managed ffmpeg downloads. |

Choose one primary feature and one small supporting item per milestone. Avoid
starting auto-update, social downloaders, and a major UI rewrite together.

## Shipped Foundation

| ID | Feature | Status | Notes |
| --- | --- | --- | --- |
| STM-00 | Steam trailer discovery by App ID or URL | shipped | Uses Steam `appdetails`; handles age-gated titles. |
| STM-00A | Steam game-name search | shipped | User selects the correct Steam match. |
| STM-00B | Trailer selection and MP4 download | shipped | ffmpeg remuxes DASH without re-encoding. |
| APP-00 | Tauri desktop application | shipped | Windows and Linux package builds are operational. |
| APP-00A | Native download-folder picker | shipped | Defaults to the OS Downloads folder. |
| APP-00B | Live download progress | shipped | Based on ffprobe duration and ffmpeg progress. |
| APP-00C | Search/download history | shipped | Durable user-local JSON, capped at 200 entries, with one-time legacy WebView recovery. |
| REL-00 | Windows and Linux CI packages | shipped | MSI, `.deb`, and `.rpm` build successfully. |

## Release and Distribution

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
| --- | --- | ---: | ---: | ---: | --- |
| REL-01 | Clean beta release baseline | shipped | P0 | M | Completed in `v0.1.1`: tag and artifact provenance aligned, lockfile committed, duplicate assets removed, and release notes updated. |
| REL-02 | Single Windows MSI distribution | shipped | P1 | S | MSI is the only public Windows installer. Preserve the original WiX UpgradeCode and validate install/update/uninstall against prior releases. |
| REL-03 | Portable Windows ZIP | planned | P1 | M | Package app plus required tools and notices; portable builds receive update notifications but do not overwrite themselves. |
| REL-04 | Signed application auto-update | planned | P1 | L | Detailed in `repo-update.md`; requires MSI elevation handling, signing keys, tag-driven releases, and update UI. |
| REL-05 | Windows Authenticode signing | dropped | P1 | M | SignPath Foundation declined the free OSS application; a paid SignPath subscription is out of scope. No code-signing plan is active. See `docs/CODE_SIGNING_POLICY.md`. |
| REL-06 | Stable and beta update channels | idea | P2 | M | Separate signed manifests; stable installations must ignore prereleases. |
| REL-07 | Linux AppImage | idea | P2 | M | Enables Tauri-style Linux self-update; retain `.deb`/`.rpm` for package-manager users. |
| REL-08 | Linux package runtime test matrix | ready | P0 | M | Test Ubuntu/Debian and Fedora clean installs, ffmpeg discovery, picker, download, uninstall. |
| REL-09 | macOS app/DMG support | deferred | P3 | L | Requires Apple hardware/runner, signing, notarization, icons, and runtime validation. |
| REL-10 | Checksums and software bill of materials | in progress | P1 | M | Tagged publisher now generates `SHA256SUMS`; signed manifests and SBOM/provenance remain. |

## Dependencies and Tooling

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
| --- | --- | ---: | ---: | ---: | --- |
| DEP-01 | Install managed ffmpeg and ffprobe on Windows | shipped | P1 | M | Downloads the latest BtbN LGPL ZIP after user approval, verifies it against the release checksum manifest, and extracts only ffmpeg/ffprobe under user-local app data. |
| DEP-02 | Detect and explain missing Linux ffmpeg | planned | P1 | S | Show distro-appropriate commands and retry detection; do not silently install system packages. |
| DEP-03 | Tool version display and diagnostics | planned | P2 | S | ffmpeg version now appears on Home and Settings; add ffprobe and future yt-dlp versions plus broader diagnostics later. |
| DEP-04 | Verified yt-dlp baseline bundle | planned | P1 | M | Implement only when the first social downloader ships; keep shipped copy as fallback. |
| DEP-05 | Independent verified yt-dlp updater | idea | P2 | L | Official source only, checksum/signature validation, staged activation, health check, rollback. |
| DEP-06 | Dependency lockfiles and reproducible CI | shipped | P0 | S | `package-lock.json` and `Cargo.lock` are committed; release CI uses `npm ci`, validates versions, and pins critical Actions. |
| DEP-07 | Third-party notices screen/file | ready | P0 | S | Required before redistributing ffmpeg or yt-dlp binaries. |

## Steam Features

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
| --- | --- | ---: | ---: | ---: | --- |
| STM-01 | Trailer quality selection | planned | P2 | M | Parse DASH variants and expose quality preferences. |
| STM-02 | JSON metadata sidecar | ready | P1 | S | Include game/app ID, trailer name, source URL, fetch time, dimensions, duration, and file hash. |
| STM-03 | Screenshot and key-art downloader | ready | P1 | M | Use screenshots, header, capsule, background, and logo data already available from Steam. |
| STM-04 | Batch game input | ready | P1 | M | Accept pasted lists, CSV, or multiple App IDs; feeds the shared queue. |
| STM-05 | Retry with exponential backoff | planned | P1 | M | Cover Steam API, manifest, and segment failures with bounded retries and cancellation. |
| STM-06 | Preview before download | idea | P2 | M | Play trailer or show thumbnails before selecting assets. |
| STM-07 | Language and region selection | idea | P3 | M | Make Steam `cc` and `l` configurable. |
| STM-08 | Steam Workshop asset support | idea | P3 | XL | Separate legal/API and content-type investigation required. |
| STM-09 | Store description and press facts export | idea | P2 | M | Export supported text metadata for review notes without copying user/private content. |
| STM-10 | Paste Steam URL from clipboard | shipped | P1 | S | Compact clipboard, clear, and search icons share the input; clipboard text is read only after a user click, while native keyboard paste remains available when the field is focused. |
| STM-11 | Safe per-game trailer folder | shipped | P1 | S | Downloads now use `<SteamID> <Safe_Game_Name>` under the selected root, reuse an existing folder with the same Steam ID, remove unsafe/special characters, and cap the title component at 100 characters. |

## Download Workflow

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
| --- | --- | ---: | ---: | ---: | --- |
| APP-01 | Common downloader-module interface | planned | P1 | L | Normalise source detection, item metadata, selection, progress, cancellation, and results. |
| APP-02 | Persistent settings | planned | P1 | M | Download folder, quality, naming, tool paths, channel, and update preferences. |
| APP-03 | Download queue and batch mode | ready | P1 | L | Add queue states, concurrency limits, pause/cancel/retry, and session recovery. |
| APP-04 | Concurrent downloads | idea | P2 | M | Requires limits and bandwidth/error behavior; start conservatively. |
| APP-05 | Resume interrupted downloads | idea | P2 | L | Depends on source/range support and safe partial-file tracking. |
| APP-06 | Duplicate detection across runs | planned | P1 | M | Use source identity plus optional hashes; distinguish skip, replace, and versioned copy. |
| APP-07 | Naming templates | idea | P2 | M | Tokens such as game, asset type, date, resolution, and source. |
| APP-08 | Automatic folder organisation | planned | P1 | M | Build on the shipped `STM-11` game folder with `Trailers`, `Screenshots`, `Art`, and `Metadata` subfolders plus a configurable template. |
| APP-09 | Disk-space and destination validation | planned | P1 | S | Check writable destination and estimated space before starting. |
| APP-10 | In-app logs and diagnostics export | idea | P2 | M | Redact usernames/paths where possible; make support reports user-controlled. |
| APP-11 | Native notifications | idea | P3 | S | Completion/failure notifications, configurable and non-spammy. |
| APP-12 | Theme picker and editor | shipped | P3 | M | Includes Default and NavyWhite1, seven-colour editing, Save As/update/rename/delete, protected Default, and a startup-theme toggle. |
| APP-13 | Keyboard and screen-reader accessibility | planned | P1 | M | Focus order, labels, contrast, reduced motion, and keyboard-complete workflows. |

## Additional Platforms

These should use yt-dlp behind the shared downloader interface rather than
custom page scrapers. Authentication bypasses and DRM circumvention are out of
scope.

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
| --- | --- | ---: | ---: | ---: | --- |
| SRC-01 | YouTube downloader | planned | P1 | L | First yt-dlp module; quality/format selection, metadata, subtitles, playlists policy. |
| SRC-02 | TikTok downloader | planned | P2 | M | Public URLs only initially; test frequent upstream changes. |
| SRC-03 | Instagram downloader | planned | P2 | L | Public content initially; authentication and cookies need a separate security design. |
| SRC-04 | Facebook downloader | planned | P2 | L | Public content initially; expect upstream variability. |
| SRC-05 | Generic yt-dlp URL handler | idea | P2 | M | Powerful but harder to support; expose detected extractor and clear compatibility status. |
| SRC-06 | Subtitle/caption download | idea | P2 | M | Pairs with SRC-01 as a "fetch captions" checkbox on YouTube. yt-dlp fetches manual (`--write-subs`) and auto-generated (`--write-auto-sub`) captions; note which kind was retrieved since auto-caption accuracy varies. Post-process VTT/SRT into two outputs: a clean, timestamp-free transcript (`.txt`) sized for feeding an AI harness reviewing the video's content, and a lightweight `[mm:ss] text` timestamped variant for citing specific moments. Needs a language-selection default (video's original audio language, with a picker) and a per-video output folder alongside the download, matching the `STM-11` per-game folder convention. |
| SRC-07 | Playlist/channel batch import | idea | P3 | L | Requires safeguards against unexpectedly large jobs. |

## Creator Workflow

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
| --- | --- | ---: | ---: | ---: | --- |
| CRT-01 | Contact sheet generation | idea | P1 | M | Create timestamped visual sheets for quickly reviewing trailers/screenshots. |
| CRT-02 | Proxy and thumbnail generation | planned | P2 | L | Presets optimised for fast preview and Resolve import. |
| CRT-03 | DaVinci Resolve bin export | idea | P2 | L | Start with folder/bin metadata before attempting timeline generation. |
| CRT-04 | DaVinci Resolve timeline export | idea | P3 | XL | Requires a clearly defined editorial workflow and interchange format. |
| CRT-05 | Asset favorites and review status | idea | P2 | M | Mark shortlist/rejected/used without modifying source media. |
| CRT-06 | Copy attribution/source details | idea | P2 | S | Copy source URL, game, publisher, and fetch date for notes/descriptions. |
| CRT-07 | Asset manifest export | idea | P1 | M | Machine-readable inventory with relative paths, metadata, and hashes. |
| CRT-08 | Thumbnail composition workspace | idea | P3 | XL | Crop, logo placement, safe areas, and export presets; potentially a separate module. |

## Creative Media and PressKit

This proposal covers importing local or remote media, removing backgrounds,
and assembling reusable press-kit graphics. Keep the background-removal tools
useful outside the PressKit editor so creators can process a single asset
without first creating a project.

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
| --- | --- | ---: | ---: | ---: | --- |
| MED-01 | Local file and URL media import | idea | P2 | M | Accept local uploads or URLs for JPEG, PNG, and WebM initially. Remote imports need scheme allowlisting, redirects and size limits, timeouts, content-type verification, and clear source attribution. |
| MED-02 | Automatic background removal | idea | P2 | L | Remove the complete background from an image using a replaceable local or hosted segmentation engine. Decide model licensing, download size, offline behavior, supported hardware, and privacy before promotion to `ready`. |
| MED-03 | Manual mask and selection refinement | idea | P2 | XL | Let users select only part of an asset, add/subtract from the mask, refine edges, preview transparency, and undo changes. Depends on MED-02 and an accessible canvas interaction model. |
| MED-04 | Transparent asset export | idea | P2 | M | Export processed still images as transparent PNG or WebP without overwriting the source. Preserve useful dimensions and metadata selectively; validate alpha handling across target applications. |
| PKT-01 | PressKit folder and project import | idea | P2 | M | Point the app at a media/assets folder, inventory supported files, preserve relative paths, and save a portable project manifest. Define missing-file and duplicate behavior. |
| PKT-02 | Layered PressKit canvas editor | idea | P2 | XL | Compose JPEG, PNG, and animated GIF layers with ordering, positioning, scaling, cropping, opacity, snapping, undo/redo, and reusable output-size presets. Depends on a deliberate canvas/rendering-engine choice. |
| PKT-03 | Animated GIF and WebM composition | idea | P3 | XL | Preview and layer animated media with explicit duration, frame-rate, loop, audio, memory, and export rules. Build after the still-image editor is reliable. |
| PKT-04 | Text layers and system font browser | idea | P2 | L | Add editable text layers using installed fonts, with search, style controls, fallbacks, and missing-font warnings. Projects should reference fonts rather than embed or redistribute them by default. |
| PKT-05 | Controlled font installation | idea | P3 | L | Install a user-selected font only after previewing its source and license. Prefer per-user installation, require explicit confirmation, reject unsupported files, and never silently elevate privileges. OS behavior differs on Windows and Linux. |

### Feasibility Review (2026-08-28)

| IDs | Verdict | Validation notes |
| --- | --- | --- |
| STM-10 | Straightforward | The existing field and parser already accept Steam store URLs. Tauri v2 provides clipboard text reading on Windows and Linux through its official clipboard-manager plugin and a specific `allow-read-text` capability. |
| MED-01, MED-04 | Straightforward with safeguards | Local file import and alpha-preserving PNG/WebP export are supported by mature image libraries. URL import needs HTTPS-only defaults, response-size and timeout limits, MIME/content validation, and non-destructive output. WebM is video input and must follow the animated pipeline rather than the still-image path. |
| MED-02 | Doable with an engine decision | A local segmentation model can run through ONNX Runtime or a native inference library. Model license, package size, CPU/GPU performance, offline download behavior, and output quality need a prototype before choosing the engine. |
| MED-03 | Doable, substantial UI work | Brush/select, mask add/subtract, edge refinement, zoom, undo/redo, and accessible keyboard alternatives are established canvas operations. This is larger than automatic removal and should follow MED-02. |
| PKT-01 | Straightforward | Tauri can select a folder and Rust can inventory supported files and write a relative-path project manifest. Define symlink, missing-file, duplicate, and external-file behavior. |
| PKT-02, PKT-04 | Doable with a canvas library | Established canvas libraries support layered images, transforms, clipping, serialization, and editable text. System-font discovery needs a small OS-specific Rust service and fallback handling for projects opened on another machine. |
| PKT-03 | Doable, highest technical risk | Animated decode, synchronized preview, memory limits, timeline controls, and GIF/WebM encoding are feasible but significantly more complex than still-image composition. Prototype after PKT-02 and use ffmpeg for final encoding where practical. |
| PKT-05 | Doable, platform-specific and sensitive | Windows and Linux support per-user font installation, but mechanisms and refresh behavior differ. Require a local font file, format validation, license/source preview, explicit confirmation, and no elevation; defer system-wide installation. |

All submitted ideas are technically feasible. With `STM-10` shipped, the
recommended creative-media order is `MED-01`, `MED-04`, `PKT-01`, `MED-02`,
`MED-03`, `PKT-02`/
`PKT-04`, then `PKT-03` and `PKT-05` after focused prototypes.

## Reliability, Security, and Privacy

| ID | Feature | Status | Priority | Effort | Dependencies / decision notes |
| --- | --- | ---: | ---: | ---: | --- |
| QLT-01 | Rust unit tests for pure logic | ready | P0 | M | URL/App ID parsing, filename safety, selection, response mapping, and version logic. |
| QLT-02 | Frontend interaction tests | planned | P1 | M | Search states, selection, history, progress, errors, and settings. |
| QLT-03 | Packaged-app smoke tests | planned | P1 | L | Clean Windows and Linux VMs; launch, fetch, download, and uninstall. |
| QLT-04 | CI formatting/lint/test gates | ready | P0 | M | Add Rust formatting/clippy/tests and frontend checks before packaging. |
| SEC-01 | Project license | complete | P0 | S | MIT license added with matching npm and Cargo metadata. |
| SEC-02 | Dependabot and dependency review | ready | P0 | S | Enable alerts; add scheduled dependency updates and PR review. |
| SEC-03 | Release provenance attestation | idea | P1 | M | GitHub artifact attestations/SLSA provenance for public binaries. |
| SEC-04 | Privacy and network-requests page | complete | P1 | S | `PRIVACY.md` covers Steam/GitHub requests, local data, and no telemetry. |
| SEC-05 | Content-use responsibility notice | planned | P1 | S | Explain that users are responsible for rights, platform terms, and redistribution. |
| SEC-06 | Secret and binary scanning in CI | idea | P1 | M | Secret scanning, dependency review, and optional malware scan of release binaries. |

## Deliberately Deferred or Out of Scope

| ID | Feature | Status | Reason |
| --- | --- | --- | --- |
| OUT-01 | macOS release | deferred | Focus remains Windows and Linux until those paths are stable. |
| OUT-02 | Built-in telemetry | deferred | No analytics unless explicitly designed as opt-in and privacy-preserving. |
| OUT-03 | DRM bypass | rejected | Not part of the product. |
| OUT-04 | Private-account credential capture | rejected | High security and platform-policy risk; public media is the initial scope. |
| OUT-05 | Automatic system-package installation on Linux | rejected | The app may explain commands but should not silently elevate or mutate the OS. |

## Idea Intake Template

Add ideas to the relevant section using this format:

```markdown
| NEW-00 | Short feature name | idea | P2 | M | User value, constraints, and dependencies. |
```

Before promoting an idea to `ready`, answer:

1. Who is it for and what repeated problem does it solve?
2. What is the smallest useful version?
3. What existing feature or architecture does it depend on?
4. What privacy, security, licensing, or platform risks apply?
5. How will we verify it works on the supported operating systems?

## Decisions Needed

- Should the next public version focus on release integrity or a new creator
  feature?
- Which Steam enhancement should come first: art, metadata, or batch queues?
- Is YouTube the first social source, or should the shared downloader interface
  be completed and tested before any new source?
- Should Linux add AppImage or remain `.deb`/`.rpm` only?
