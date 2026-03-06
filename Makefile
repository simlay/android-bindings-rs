


run-device:
	cargo apk run --target aarch64-linux-android  -p simple --no-logcat

logs:
	adb logcat RustStdoutStderr:V 'com.simlay.example:V' simple:V '*:S'

watch:
	cargo watch -s 'make run-device' -w build.rs -w src/ -w Cargo.toml -w examples/ -w java/

screenshot: run-device
	sleep 2
	adb shell screencap /sdcard/screenshot.png
	adb pull /sdcard/screenshot.png
