use std::{
    borrow::Cow,
    error::Error,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use jaffi::Jaffi;
use std::fs;
use std::io;

fn extract_jar(file: PathBuf) {
    let file = fs::File::open(file).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    //let output_dir = PathBuf::from(std::env::var("CARGO_TARGET_DIR").unwrap()).join("android-src");
    let output_dir = PathBuf::from("./target").join("android-src");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => output_dir.join(path),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

fn class_path(jar: Option<String>) -> PathBuf {

    let android_jar = if let Some(jar) = jar {
        PathBuf::from(jar)
    } else {
        let android_home =
            PathBuf::from(std::env::var("ANDROID_HOME").expect("ANDROID_HOME not set"));
        android_home.join("platforms/android-35/android.jar")
    };
    if !std::path::Path::new("./target/android-src/").exists() {
        extract_jar(android_jar);
    }
    PathBuf::from("target/android-src/")
}
fn build_dex() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let android_jar_path = android_build::android_jar(None)
            .expect("Failed to find android.jar");

        // Compile Java → .class
        android_build::JavaBuild::new()
            .class_path(&android_jar_path)
            .classes_out_dir(std::path::PathBuf::from(&out_dir))
            .file("java/com/example/NativeRunnable.java")
            .compile()
            .expect("javac failed");

        // DEX the .class → classes.dex
        android_build::Dexer::new()
            .out_dir(std::path::PathBuf::from(&out_dir))
            .file(
                std::path::PathBuf::from(&out_dir)
                    .join("com/example/NativeRunnable.class")
            )
            .run()
            .expect("d8 failed");

        // Tell cargo to re-run if the java source changes
        println!("cargo:rerun-if-changed=java/com/example/NativeRunnable.java");
}

fn main() -> Result<(), Box<dyn Error>> {
    build_dex();

    let android_source = class_path(std::env::var("ANDROID_JAR").ok());
    //let androidx_fragment = class_path(Some("fragment-1.6.0-sources.jar".to_string()));
    let classes = vec![
        ////Cow::from("android.annotation.AttrRes"),
        //Cow::from("com.example.NativeRunnable"),
    ];
    let classes_to_wrap = vec![

        Cow::from("android.R"),
        Cow::from("android.app.Activity"),
        Cow::from("android.app.NativeActivity"),
        Cow::from("android.util.AndroidException"),
        Cow::from("android.util.AttributeSet"),
        Cow::from("android.content.IntentSender"),
        Cow::from("android.view.ContextThemeWrapper"),
        Cow::from("android.view.SurfaceView"),
        // This results in a duplicate exception field in the enum: Cow::from("android.os.Debug"),

        // Works
        Cow::from("android.view.KeyEvent"),
        Cow::from("android.view.Window"),
        Cow::from("android.view.ViewGroup"),
        Cow::from("android.view.ViewGroup$LayoutParams"),
        Cow::from("android.view.ViewManager"),
        Cow::from("android.view.WindowManager"),
        Cow::from("android.view.WindowManager$LayoutParams"),
        Cow::from("android.graphics.drawable.Drawable"),
        Cow::from("android.graphics.Color"),
        Cow::from("android.widget.EditText"),
        Cow::from("android.widget.TextView"),
        Cow::from("android.widget.RelativeLayout"),
        Cow::from("android.widget.LinearLayout"),
        Cow::from("android.widget.FrameLayout"),
        Cow::from("android.widget.PopupWindow"),
        /*
        */
        //Cow::from("android.view.LayoutInflater"),
        Cow::from("android.widget.Button"),
        Cow::from("android.view.autofill.AutofillId"),
        Cow::from("android.view.View"),
        Cow::from("android.view.autofill.AutofillManager"),
        // AndroidX
        //Cow::from("androidx.fragment.app.FragmentActivity"),

        // Java Defaults
        Cow::from("java.lang.CharSequence"),
        Cow::from("java.lang.Runnable"),
        Cow::from("java.lang.Exception"),
        Cow::from("java.util.ArrayList"),
        //Cow::from("java.lang.ClassLoader"),
        Cow::from("dalvik.system.InMemoryDexClassLoader"),
        //Cow::from("java.io.PrintWriter"),
        //Cow::from("java.lang.String"),
        //Cow::from("android.view.Surface"),
    ];
    let output_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    let output_file = Cow::from(Path::new("generated_jaffi.rs"));

    let jaffi = Jaffi::builder()
        .output_dir(&output_dir)
        .path_modules(true)
        //.output_filename(&output_file)
        .native_classes(classes)
        .classes_to_wrap(classes_to_wrap)
        .classpath(vec![
            Cow::from(android_source),
            Cow::from(output_dir.join("javac-build/classes")),
        ])
        .build();

    jaffi.generate()?;

    // let's format the file to help with debugging build issues
    let jaffi_file = output_dir.join(output_file);

    let mut cmd = Command::new("rustfmt");
    cmd.arg("--emit").arg("files").arg(jaffi_file);

    eprintln!("cargo fmt: {cmd:?}");
    let output = cmd.output();

    match output {
        Ok(output) => {
            std::io::stderr().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
        }
        Err(e) => {
            eprintln!("cargo fmt failed to execute: {e}");
        }
    }

    Ok(())
}
