# Gradle Example

This example demonstrates using the Android bindings with `cargo-ndk` and a standard Gradle build (not cargo-apk).

## Requirements

- Android NDK installed
- ANDROID_HOME environment variable set
- `cargo-ndk` installed: `cargo install cargo-ndk`

## Building

From the `examples/gradle` directory:

```bash
# Build with Gradle (automatically runs cargo-ndk and packages the library)
./gradlew assembleDebug

# Or build with cargo-ndk directly
cargo ndk build --target arm64-v8a --release
```

## Running

```bash
# Install the APK
adb install -r app/build/outputs/apk/debug/app-debug.apk

# Run the app on your device
adb shell am start -n com.simlay.gradle/.GradleActivity

# View logs
adb logcat | grep -E "RUST|gradle|libgradle"
```

## What This Demonstrates

- Using `cargo-ndk` instead of cargo-apk
- Standard Java Activity as entry point (not NativeActivity)
- Loading Rust library via JNI
- Pure Android UI creation (Button)
- No winit or event loop required

## Project Structure

```
examples/gradle/
├── Cargo.toml           # Rust package config
├── src/
│   └── lib.rs           # Rust JNI entry points
├── app/
│   ├── build.gradle     # Gradle build script
│   └── src/main/
│       ├── AndroidManifest.xml
│       └── java/com/example/
│           └── GradleActivity.java
└── README.md
```

## Key Differences from examples/simple

| Feature | examples/simple | examples/gradle |
|---------|-----------------|-----------------|
| Build system | cargo-apk | Gradle + cargo-ndk |
| Entry point | NativeActivity (native) | Java Activity |
| Event loop | winit | Native Android |
| Complexity | High (event loop, rendering) | Minimal (just UI) |
| Use case | Graphics/event-driven apps | Simple UI apps |
