package com.wannazif.solidbreak

import android.app.NativeActivity
import android.os.Bundle
import android.system.Os
import java.io.File

class SolidBreakActivity : NativeActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        // Stage HOME and XDG_DATA_HOME before NativeActivity loads libsolidbreak_native.so
        val appFilesDir = filesDir.absolutePath
        val xdgDataHome = File(filesDir, ".local/share")
        val fontsDir = File(xdgDataHome, "fonts")
        if (!fontsDir.exists()) {
            fontsDir.mkdirs()
        }

        try {
            Os.setenv("HOME", appFilesDir, true)
            Os.setenv("XDG_DATA_HOME", xdgDataHome.absolutePath, true)
            Os.setenv("XDG_CONFIG_HOME", File(filesDir, ".config").apply { mkdirs() }.absolutePath, true)
            Os.setenv("XDG_CACHE_HOME", cacheDir.absolutePath, true)
        } catch (_: Throwable) {
            // Ignore if already set
        }

        stageBundledFonts(fontsDir)
        super.onCreate(savedInstanceState)
    }

    private fun stageBundledFonts(destDir: File) {
        try {
            val assetList = assets.list("") ?: return
            for (name in assetList) {
                if (name.endsWith(".ttf", ignoreCase = true) || name.endsWith(".otf", ignoreCase = true)) {
                    val outFile = File(destDir, name)
                    if (!outFile.exists()) {
                        assets.open(name).use { input ->
                            outFile.outputStream().use { output ->
                                input.copyTo(output)
                            }
                        }
                    }
                }
            }
        } catch (_: Throwable) {
            // Optional bundled fonts in Phase 1
        }
    }
}
