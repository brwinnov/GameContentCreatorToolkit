# Contributing

Contributions should stay focused on the supported Windows and Linux desktop
application, the Steam media workflow, release integrity, or the documented
roadmap. DRM bypass and private-account credential capture are out of scope.

## Before Opening a Pull Request

1. Open or reference an issue for behavioral changes when practical.
2. Keep changes narrow and update relevant documentation or tests.
3. Work in the `app/` Tauri project and keep the desktop app as the active path.
4. Never commit downloaded media, build output, logs, credentials, or `.env`.

Run the applicable checks from the repository root:

```powershell
Set-Location app/src-tauri
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
Set-Location ../..
node --check app/src/main.js
```

Also lint changed Markdown and confirm local links still resolve. Release
workflow changes require explicit maintainer review. By contributing,
you agree that your contribution is provided under the repository's
[MIT License](LICENSE).
