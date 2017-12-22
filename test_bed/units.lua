require "tundra.syntax.glob"
require "tundra.syntax.qt"
require "tundra.syntax.rust-cargo"
require "tundra.path"
require "tundra.util"

local native = require('tundra.native')

local function gen_moc(src)
    return Moc {
        Pass = "GenerateSources",
        Source = src
    }
end

-----------------------------------------------------------------------------------------------------------------------

SharedLibrary {
    Name = "wrui_qt",
    Sources = {
        Glob {
            Dir = "src/qt",
            Extensions = { ".cpp", ".h" },
            Recursive = true,
        },

        gen_moc("test_bed/qt/wrui_qt.h"),
    },

    Env = {
       CXXOPTS = {
            { "-isystem $(QT5)/lib/QtWidgets.framework/Headers",
              "-isystem $(QT5)/lib/QtCore.framework/Headers",
              "-isystem $(QT5)/lib/QtGui.framework/Headers",
              "-F$(QT5)/lib"; Config = "macosx-*-*" },

            { "-isystem $(QT5)/include/QtWidgets",
              "-isystem $(QT5)/include/QtCore",
              "-isystem $(QT5)/include/QtGui",
              "-isystem $(QT5)/include"; Config = "linux-*-*" },
        },

        CPPDEFS = {
            "QT_NO_KEYWORDS",
            "QT_NO_CAST_FROM_ASCII",
            "QT_NO_CAST_TO_ASCII",
        },

        CPPPATH = {
            "$(QT5)/include",
            "$(QT5)/include/QtCore",
            "$(QT5)/include/QtGui",
            "$(QT5)/include/QtWidgets",
        },

        LIBPATH = {
			{ "$(QT5)\\lib"; Config = "win64-*-*" },
			{ "$(QT5)/lib"; Config = "linux-*-*" },
		},

        PROGCOM = {
            {  "-Wl,-rpath,$(QT5)/lib", "-F$(QT5)/lib", "-lstdc++", Config = "macosx-clang-*" },
            {  "-Wl,-rpath,$(QT5)/lib", "-lstdc++", "-lm", Config = "linux-*-*" },
        },
    },

	Libs = {
		{ "wsock32.lib", "kernel32.lib", "user32.lib", "gdi32.lib", "Comdlg32.lib",
		  "Advapi32.lib", "Qt5Gui.lib", "Qt5Core.lib", "Qt5Widgets.lib"; Config = "win64-*-*" },
		{ "Qt5Gui", "Qt5Core", "Qt5Widgets"; Config = "linux-*-*" },
	},

    Frameworks = { "Cocoa", "QtWidgets", "QtGui", "QtCore" },
}

Default "wrui_qt"

-- vim: ts=4:sw=4:sts=4

