
LLVM_PATH=/Users/simlay/Library/Android/sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/darwin-x86_64/bin


run-device:
	cargo apk run --target aarch64-linux-android  -p simple --no-logcat

xbuild:
	PATH=$(LLVM_PATH):${PATH} JAVA_HOME=/opt/homebrew/opt/openjdk/ x build --device 'adb:pixel-xl.house.simlay.net:5555' --arch arm64 -p simple -v

xbuild-install: xbuild
	adb install target/x/debug/android/simple.apk

xbuild-run: xbuild-install
	adb shell am start -n com.simlay.example/com.simlay.example.MainActivity

uninstall:
	adb uninstall com.simlay.example

logs-recent:
	adb logcat RustStdoutStderr:V 'com.simlay.example:V' simple:V 'com.simlay.example' '*:S' -n 1000

logs-follow:
	adb logcat RustStdoutStderr:V 'com.simlay.example:V' simple:V 'com.simlay.example' '*:S'

watch:
	cargo watch -s 'make run-device' -w build.rs -w src/ -w Cargo.toml -w examples/ -w java/

scrcpy:
	scrcpy --video-codec=h265 --max-size=1920 --max-fps=60 --no-audio --keyboard=uhid

screenshot: run-device
	sleep 2
	adb shell screencap /sdcard/screenshot.png
	adb pull /sdcard/screenshot.png
