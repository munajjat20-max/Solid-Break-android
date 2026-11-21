# Solid Break (Pure Native Rust Android CAD)

100% pure-native Rust Android port of [SolidCraft](https://github.com/tangency-labs/solidcraft) (`egui 0.37` + `eframe` + `wgpu 30` + `android-activity 0.6` + Kotlin `NativeActivity`).

- **Zero WebView, zero WASM, zero JavaScript**
- **100% Offline** (zero `INTERNET` permission)
- **Permissive Open-Source Licenses Only** (`MIT OR Apache-2.0`)
- **CI-Only Builds** via GitHub Actions (`cargo ndk` + Gradle 8.11.1)

## Phase 1: Minimal Bootable Native APK (`arm64-v8a`)
1. Push this scaffold to your `Solid-Break` GitHub repository.
2. Open the **Actions** tab on GitHub and watch the **Build Solid Break (Phase 1 Debug)** workflow.
3. Download `solid-break-phase1-arm64-debug` from the workflow artifacts and install it on your tablet (e.g., OnePlus Pad 3) to verify Vulkan rendering.
4. Paste the CI log (or any compiler errors) back to continue to Phase 2 (Native SAF File Picker + Tablet Touch Controls Bar).
