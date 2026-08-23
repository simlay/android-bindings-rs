use jbindgen::Builder;
use std::fs;
use std::path::PathBuf;

static DEFAULT_API_LEVEL: u32 = 31;

fn build_dex() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let api_level = std::env::var("ANDROID_API_LEVEL")
        .map(|v| v.parse::<u32>().unwrap_or(DEFAULT_API_LEVEL))
        .unwrap_or(DEFAULT_API_LEVEL);

    // Get Android SDK jar for compilation
    let android_home = std::env::var("ANDROID_HOME")
        .or_else(|_| std::env::var("ANDROID_SDK_ROOT"))
        .expect("ANDROID_HOME or ANDROID_SDK_ROOT not set");
    let android_jar = PathBuf::from(&android_home)
        .join("platforms")
        .join(format!("android-{}", api_level))
        .join("android.jar");


    // Compile Java NativeRunnable to class file
    let java_src_dir = PathBuf::from("java");
    let javac_out_dir = PathBuf::from(&out_dir).join("javac-build/classes");
    fs::create_dir_all(&javac_out_dir).expect("Failed to create build directory");

    // Compile NativeRunnable.java using javac crate
    let compiled_files = javac::Build::new()
        .source_dir(&java_src_dir)
        .output_dir(&javac_out_dir)
        .release("21")
        .classpath(&android_jar)
        .compile();


    // DEX the .class → classes.dex
    android_build::Dexer::new()
        .out_dir(std::path::PathBuf::from(&out_dir))
        .files(compiled_files)
        .run()
        .expect("d8 failed");

    // Tell cargo to re-run if the java source changes
    println!("cargo:rerun-if-changed=java/com/example/NativeRunnable.java");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;

    build_dex();

    let api_level = std::env::var("ANDROID_API_LEVEL")
        .map(|v| v.parse::<u32>().unwrap_or(DEFAULT_API_LEVEL))
        .unwrap_or(DEFAULT_API_LEVEL);


    // Generate bindings using jbindgen
    // Exclude core Java types that are automatically mapped by the jni crate
    // and problematic packages like java.util.zip, java.util.stream, etc.
    let patterns = vec![
        "android.R".to_string(),
        "android.app.Activity".to_string(),
        "android.app.NativeActivity".to_string(),
        "android.util.*".to_string(),
        "android.content.*".to_string(),
        "android.view.View".to_string(),
        "android.view.SurfaceView".to_string(),
        "android.graphics.*".to_string(),
        "android.widget.*".to_string(),
        "android.view.autofill.*".to_string(),
        "android.os.*".to_string(),
        //"android.content.pm.*".to_string(),
    ];

    let patterns = vec![

        "android.R".to_string(),
        "android.app.Activity".to_string(),
        "android.app.NativeActivity".to_string(),
        "android.util.AndroidException".to_string(),
        "android.util.AttributeSet".to_string(),
        "android.content.IntentSender".to_string(),
        "android.view.ContextThemeWrapper".to_string(),
        "android.view.SurfaceView".to_string(),
        // This results in a duplicate exception field in the enum: Cow::from("android.os.Debug"),

        // Works
        "android.view.KeyEvent".to_string(),
        "android.view.Window".to_string(),
        "android.view.ViewGroup".to_string(),
        "android.view.ViewGroup$LayoutParams".to_string(),
        "android.view.ViewManager".to_string(),
        "android.view.WindowManager".to_string(),
        "android.view.WindowManager$LayoutParams".to_string(),
        "android.graphics.drawable.Drawable".to_string(),
        "android.graphics.drawable.ColorDrawable".to_string(),
        "android.graphics.Color".to_string(),
        "android.widget.EditText".to_string(),
        "android.widget.TextView".to_string(),
        "android.widget.RelativeLayout".to_string(),
        "android.widget.LinearLayout".to_string(),
        "android.widget.FrameLayout".to_string(),
        "android.widget.PopupWindow".to_string(),
        /*
        */
        //Cow::from("android.view.LayoutInflater"),
        "android.widget.Button".to_string(),
        "android.view.autofill.AutofillId".to_string(),
        "android.view.View".to_string(),
        "android.view.autofill.AutofillManager".to_string(),
        // AndroidX
        //Cow::from("androidx.fragment.app.FragmentActivity"),

        // Java Defaults
        "java.lang.CharSequence".to_string(),
        "java.lang.Runnable".to_string(),
        "java.lang.Exception".to_string(),
        "java.util.ArrayList".to_string(),
        //Cow::from("java.lang.ClassLoader"),
        "dalvik.system.InMemoryDexClassLoader".to_string(),
    ];
    /*
        */

    // Generate bindings using jbindgen
    let bindings = Builder::new()
        .input_android_sdk(api_level, patterns)
        .skip_signature("Landroid/view/WindowManager$LayoutParams;->type:I")
        .skip_signature("Landroid/R$transition;->move:I")
        .skip_signature("Landroid/R$attr;->type:I")
        .skip_signature("Landroid/os/PatternMatcher;->match(Ljava/lang/String;)I")
        //.skip_signature("Landroid/content/IntentFilter;->match(Ljava/lang/String;)I")
        .root_path("crate::bindings")
        .generate()?;

    bindings.write_to_files(&out_dir)?;

    // Also write the type map
    let type_map_path = PathBuf::from(&out_dir).join("type_map.rs");
    bindings.write_pub_type_map(&type_map_path, "crate::bindings")?;

    // In your lib.rs or main.rs, include the generated bindings:
    // include!(concat!(env!("OUT_DIR"), "/generated_jaffi.rs"));

    Ok(())
}
