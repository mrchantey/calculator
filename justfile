set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set dotenv-load


build-wasm:
	cd calculator-rust && just build

serve:
	cd calculator-astro && just run