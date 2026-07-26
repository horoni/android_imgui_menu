fn main() {
	cc::Build::new()
		.file("third_party/xdl/xdl/src/main/cpp/xdl_util.c")
		.file("third_party/xdl/xdl/src/main/cpp/xdl_lzma.c")
		.file("third_party/xdl/xdl/src/main/cpp/xdl_linker.c")
		.file("third_party/xdl/xdl/src/main/cpp/xdl_iterate.c")
		.file("third_party/xdl/xdl/src/main/cpp/xdl.c")
		.include("third_party/xdl/xdl/src/main/cpp/include")
		.flag("-O2")
		.compile("xdl");

	cc::Build::new()
		.cpp(true)
		.file("third_party/cimgui/imgui/imgui.cpp")
		.file("third_party/cimgui/imgui/imgui_draw.cpp")
		.file("third_party/cimgui/imgui/imgui_tables.cpp")
		.file("third_party/cimgui/imgui/imgui_widgets.cpp")
		.file("third_party/cimgui/imgui/backends/imgui_impl_vulkan.cpp")
		.file("third_party/cimgui/imgui/backends/imgui_impl_opengl3.cpp")
		.file("third_party/cimgui/imgui/backends/imgui_impl_android.cpp")
		.file("third_party/cimgui/cimgui.cpp")
		.file("third_party/cimgui/cimgui_impl.cpp")
		.include("third_party/cimgui/imgui")
		.include("third_party/cimgui/imgui/backends")
		.define("IMGUI_IMPL_VULKAN_NO_PROTOTYPES", None)
		.define("IMGUI_IMPL_OPENGL_ES3", None)
		.define("IMGUI_USER_CONFIG", Some("\"../cimconfig.h\""))
		.define("IMGUI_DISABLE_OBSOLETE_FUNCTIONS", Some("1"))
		.define("IMGUI_IMPL_API", Some("extern \"C\""))
		.define("CIMGUI_USE_VULKAN", None)
		.define("CIMGUI_USE_OPENGL3", None)
		.flag("-fno-rtti")
		.flag("-fno-exceptions")
		.flag("-fno-threadsafe-statics")
		.flag("-Wno-unused-function")
		.flag("-Wno-unused-variable")
		.flag("-O2")
		.compile("imgui");

	println!("cargo:rustc-link-lib=static=c++_static");
	println!("cargo:rustc-link-lib=GLESv3");
	println!("cargo:rustc-link-lib=EGL");
	println!("cargo:rustc-link-lib=android");
	println!("cargo:rustc-link-lib=log");
}
