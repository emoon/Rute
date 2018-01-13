require "tundra.syntax.glob"
require "tundra.syntax.qt"
require "tundra.syntax.rust-cargo"
require "tundra.path"
require "tundra.util"

local native = require('tundra.native')

-----------------------------------------------------------------------------------------------------------------------

local function gen_moc(src)
    return Moc {
        Pass = "GenerateSources",
        Source = src
    }
end

-----------------------------------------------------------------------------------------------------------------------

local function gen_uic(src)
    return Uic {
        Pass = "GenerateSources",
        Source = src
    }
end

-----------------------------------------------------------------------------------------------------------------------

local function gen_rcc(src)
    return Rcc {
        Pass = "GenerateSources",
        Source = src
    }
end

-----------------------------------------------------------------------------------------------------------------------

local function get_rs_src(dir)
    return Glob {
        Dir = dir,
        Extensions = { ".rs" },
        Recursive = true,
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

        gen_moc("src/qt/qt_api_gen.h"),
        gen_moc("src/qt/ToolWindowManager/ToolWindowManager.h"),
        gen_moc("src/qt/ToolWindowManager/ToolWindowManagerArea.h"),
        gen_moc("src/qt/ToolWindowManager/ToolWindowManagerSplitter.h"),
        gen_moc("src/qt/ToolWindowManager/ToolWindowManagerTabBar.h"),
        gen_moc("src/qt/ToolWindowManager/ToolWindowManagerWrapper.h"),
        gen_moc("src/qt/FramlessWindow/framelesswindow/framelesswindow.h"),
        gen_moc("src/qt/FramlessWindow/framelesswindow/windowdragger.h"),
        gen_moc("src/qt/FramlessWindow/DarkStyle.h"),

        gen_uic("src/qt/FramlessWindow/framelesswindow/framelesswindow.ui"),
        gen_rcc("src/qt/FramlessWindow/darkstyle.qrc"),
        -- gen_rcc("src/qt/FramlessWindow/framelesswindow.qrc"),
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
            "QT_NO_CAST_FROM_ASCII",
            "QT_NO_CAST_TO_ASCII",
        },

        CPPPATH = {
            "src/qt/ToolWindowManager/",
            "src/qt/ToolWindowManager/",
			"src/qt/FramlessWindow/framelesswindow",
			"src/qt/FramlessWindow",
            "$(QT5)/include",
            "$(QT5)/include/QtCore",
            "$(QT5)/include/QtGui",
            "$(QT5)/include/QtWidgets",
            "$(OBJECTROOT)", "$(OBJECTDIR)",
        },

        LIBPATH = {
			{ "$(QT5)\\lib"; Config = "win64-*-*" },
			{ "$(QT5)/lib"; Config = "linux-*-*" },
		},

        SHLIBOPTS = {
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

-----------------------------------------------------------------------------------------------------------------------

Program {
    Name = "cpp_test",
    Sources = {
        Glob {
            Dir = "src/cpp_test",
            Extensions = { ".cpp", ".h" },
            Recursive = true,
        },
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

-----------------------------------------------------------------------------------------------------------------------

RustCrate {
    Name = "wrui",
    CargoConfig = "src/wrui/Cargo.toml",
    Sources = {
        get_rs_src("src/wrui/src"),
    },
}

-----------------------------------------------------------------------------------------------------------------------

RustCrate {
    Name = "widgets",
    CargoConfig = "src/widgets/Cargo.toml",
    Sources = {
        get_rs_src("src/widgets/src"),
    },
}

-----------------------------------------------------------------------------------------------------------------------

Default "cpp_test"
Default "widgets"
Default "wrui_qt"
Default "wrui"

-- vim: ts=4:sw=4:sts=4

