This is a bindgen repo to for android bindings. The set of java classes that
are generated are in the `build.rs`. The output for these bindings are down in
the `target` directory.

The supported rust targets are `aarch64-linux-android`, `x86_64-linux-android`
and `armv7-linux-androideabi`. The Android SDK targets are TBD.

This includes an example of the generated bindings that are used in
`examples/simple/`. This example uses a NativeActiviy and winit (via a feature flag).


Commands:
* `make run-device` runs on devices and watches the logs.
* `make xbuild` will build using xbuild.
* `make xbuild-run` will build, install and run the app.
* `make logs-recent` will get the recent logs of the app running.
* `make watch` will rerun the build
* `make screenshot` will run the program and then take a screenshot via `adb
shell screencap` and save it locally into the screenshot.png file.

Guidelines:
* Any temporary directory should go in `target/<name>`.
* Cleaning the project should use `cargo clean` rather than `rm -rf`.
* Always use the `edit` tool over the `write` as rewriting a file is error prone.
