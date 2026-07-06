

run-device:
	cargo apk run --target aarch64-linux-android  -p simple --no-logcat
	adb logcat RustStdoutStderr:V 'com.simlay.example:V' simple:V '*:S'

watch:
	cargo watch -s 'make run-device' -w build.rs -w src/ -w Cargo.toml -w examples/ -w java/
