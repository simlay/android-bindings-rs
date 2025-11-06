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

fn class_path() -> PathBuf {
    let android_jar = if let Ok(android_jar) = std::env::var("ANDROID_JAR") {
        PathBuf::from(android_jar)
    } else {
        let android_home =
            PathBuf::from(std::env::var("ANDROID_HOME").expect("ANDROID_HOME not set"));
        android_home.join("platforms/android-34/android.jar")
    };
    if !std::path::Path::new("./target/android-src/").exists() {
        extract_jar(android_jar);
    }
    PathBuf::from("target/android-src/")
}

fn main() -> Result<(), Box<dyn Error>> {
    // only need this if you need to compile the java, this is needed for the integration tests...

    let class_path = class_path();
    let classes = vec![
        ////Cow::from("android.annotation.AttrRes"),
        //Cow::from("java.lang.String"),
    ];
    let classes_to_wrap = vec![
        //Cow::from("android.annotation.AttrRes"),
        // Does not work
        Cow::from("android.R"),
        //Cow::from("android.R.id"),
        //Cow::from("android.R.layout"),
        Cow::from("android.app.Activity"),
        Cow::from("android.util.AndroidException"),
        Cow::from("android.util.AttributeSet"),
        Cow::from("android.content.IntentSender"),
        Cow::from("android.view.ContextThemeWrapper"),
        // Works
        Cow::from("android.view.KeyEvent"),
        Cow::from("android.view.View"),
        Cow::from("android.view.Window"),
        Cow::from("android.view.ViewGroup"),
        Cow::from("android.view.ViewGroup$LayoutParams"),
        Cow::from("android.graphics.drawable.Drawable"),
        Cow::from("android.graphics.Color"),
        Cow::from("android.widget.EditText"),
        Cow::from("android.widget.TextView"),
        Cow::from("android.widget.RelativeLayout"),
        Cow::from("android.widget.LinearLayout"),
        //Cow::from("android.view.LayoutInflater"),
        Cow::from("android.widget.Button"),
        Cow::from("android.view.autofill.AutofillId"),
        Cow::from("android.view.autofill.AutofillManager"),
        Cow::from("java.lang.CharSequence"),
        //Cow::from("java.lang.String"),
        //Cow::from("android.view.Surface"),
    ];
    let output_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    let output_file = Cow::from(Path::new("generated_jaffi.rs"));

    let jaffi = Jaffi::builder()
        .output_dir(&output_dir)
        .output_filename(&output_file)
        .native_classes(classes)
        .classes_to_wrap(classes_to_wrap)
        .classpath(vec![Cow::from(class_path)])
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
