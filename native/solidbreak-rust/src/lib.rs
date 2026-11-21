//! Solid Break — Phase 1 Pure-Native Android Rust Entry Point
//!
//! NOT COMPILED YET: Verify via GitHub Actions CI (`cargo ndk -t arm64-v8a`).

#[cfg(target_os = "android")]
mod android_app {
    use solidcraft_engine::Session;
    use solidcraft_ui_egui::{Services, SolveApp};

    const DEPTH_BITS: u8 = 32;

    struct SolidBreakApp {
        solve: SolveApp,
    }

    impl SolidBreakApp {
        fn new(cc: &eframe::CreationContext<'_>) -> Self {
            solidcraft_ui_egui::fonts::setup_bundled_fonts(&cc.egui_ctx);
            solidcraft_ui_egui::gpu::install_cpu_picture_renderer();

            let mut solve = SolveApp::new(Session::default(), Services::default());

            if let Some(rs) = &cc.wgpu_render_state {
                install_android_viewport(&mut solve, rs);
            }

            solidcraft_ui_egui::prefs::apply_new_design(&mut solve);
            let rev = solve.session.revision;
            solve.home.show(rev);

            Self { solve }
        }
    }

    /// Configure the offscreen CAD depth buffer and cap mobile MSAA to 1x/2x
    /// to prevent mobile VRAM OOM crashes on Android GPUs.
    fn install_android_viewport(app: &mut SolveApp, rs: &eframe::egui_wgpu::RenderState) {
        let backend = rs.adapter.get_info().backend;
        let depth = eframe::egui_wgpu::depth_format_from_bits(DEPTH_BITS, 0);
        let supported = eframe::egui_wgpu::supported_msaa_samples(
            &rs.adapter,
            &rs.device,
            rs.target_format,
            depth,
        );
        let mobile_msaa = app.preferences.msaa.clamp(1, 2);
        let requested = solidcraft_ui_egui::gpu::scene_samples(backend, mobile_msaa, &supported);
        let drawn = app.set_wgpu_split(rs, DEPTH_BITS, 0, requested);
        app.msaa_supported = solidcraft_ui_egui::gpu::scene_sample_choices(backend, supported);
        log::info!(
            "Solid Break GPU ready: backend={backend:?}, MSAA={drawn}x (mobile cap={mobile_msaa}x)"
        );
    }

    impl eframe::App for SolidBreakApp {
        fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            // Guardrail: never call ctx.set_zoom_factor(...) globally on Android wgpu.
            self.solve.logic(ctx);
        }

        fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
            self.solve.raw_input_hook(raw);
        }

        fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
            self.solve.ui(ui);
        }
    }

    #[unsafe(no_mangle)]
    fn android_main(app: android_activity::AndroidApp) {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Info)
                .with_tag("SolidBreak"),
        );

        log::info!("Starting Solid Break (Phase 1 Native Rust Android)");

        // Install SolidCraft's global i18n, look, workspace, and navigation tables.
        solidcraft_ui_egui::i18n::install();
        solidcraft_look::install_tables();
        solidcraft_ui_egui::workspace::install_tabs();
        solidcraft_ui_egui::navigation_slots::install();

        let mut options = eframe::NativeOptions {
            android_app: Some(app),
            depth_buffer: 0, // Offscreen CAD viewport owns its own 32-bit depth buffer
            ..Default::default()
        };

        // Prefer Vulkan with GLES fallback on Android GPUs.
        if let eframe::egui_wgpu::WgpuSetup::CreateNew(ref mut create) =
            options.wgpu_options.wgpu_setup
        {
            create.instance_descriptor.backends =
                eframe::wgpu::Backends::VULKAN | eframe::wgpu::Backends::GL;
        }

        if let Err(err) = eframe::run_native(
            "Solid Break",
            options,
            Box::new(|cc| Ok(Box::new(SolidBreakApp::new(cc)))),
        ) {
            log::error!("eframe::run_native exited with error: {err}");
        }
    }
}
