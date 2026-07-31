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
}
