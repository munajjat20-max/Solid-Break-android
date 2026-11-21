# Solid Break (Pure Native Rust Android CAD)

100% pure-native Rust Android port of [SolidCraft](https://github.com/tangency-labs/solidcraft) (`egui 0.37` + `eframe` + `wgpu 30` + `android-activity 0.6` + Kotlin `NativeActivity`).

- **Zero WebView, zero WASM, zero JavaScript**
- **100% Offline** (zero `INTERNET` permission)
- **Permissive Open-Source Licenses Only** (`MIT OR Apache-2.0`)
- **CI-Only Builds** via GitHub Actions (`cargo ndk` + Gradle 8.11.1)

## Phase 1: Minimal Bootable Native APK (`arm64-v8a`)
1. Extract this scaffold into the root of your `Solid-Break` repository.
2. Push to `main` and open the **Actions** tab on GitHub to watch **Build Solid Break (Phase 1 Debug)**.
3. Download `solid-break-phase1-arm64-debug` from the workflow artifacts and install it on your tablet to verify Vulkan rendering.
