use android_activity::{
    input::{InputEvent, KeyAction, KeyEvent, KeyMapChar, MotionAction},
    AndroidApp, InputStatus, MainEvent, PollEvent,
};
use android_bindings::{
    AndroidAppActivity, AndroidContentContext, AndroidGraphicsColor,
    AndroidViewViewGroupLayoutParams, AndroidViewWindow, AndroidWidgetEditText,
    AndroidWidgetLinearLayout, AndroidWidgetLinearLayoutLayoutParams, AndroidWidgetRelativeLayout,
    AndroidViewAutofillAutofillManager,

    AndroidWidgetTextView, JavaLangCharSequence,
};
use jaffi_support::jni::{
    objects::{JObject, JString, JValue},
    strings::{JNIStr, JNIString, JavaStr},
    sys::{jbyte, jchar},
    JavaVM,
};
use log::info;
use winit::raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    info!("before hello world");
    println!("before hello world");
    let _ = ndk_context_jni_test(app.clone());
    let mut quit = false;
    let mut redraw_pending = true;
    let mut native_window: Option<ndk::native_window::NativeWindow> = None;
    while !quit {
        app.poll_events(
            Some(std::time::Duration::from_secs(1)), /* timeout */
            |event| {
                match event {
                    PollEvent::Wake => {
                        info!("Early wake up");
                    }
                    PollEvent::Timeout => {
                        info!("Timed out");
                        // Real app would probably rely on vblank sync via graphics API...
                        redraw_pending = true;
                    }
                    PollEvent::Main(main_event) => {
                        info!("Main event: {:?}", main_event);
                        match main_event {
                            MainEvent::SaveState { saver, .. } => {
                                saver.store("foo://bar".as_bytes());
                            }
                            MainEvent::Pause => {}
                            MainEvent::Resume { loader, .. } => {
                                if let Some(state) = loader.load() {
                                    if let Ok(uri) = String::from_utf8(state) {
                                        info!("Resumed with saved state = {uri:#?}");
                                    }
                                }
                            }
                            MainEvent::InitWindow { .. } => {
                                native_window = app.native_window();
                                redraw_pending = true;
                            }
                            MainEvent::TerminateWindow { .. } => {
                                native_window = None;
                            }
                            MainEvent::WindowResized { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::RedrawNeeded { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::InputAvailable { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::ConfigChanged { .. } => {
                                info!("Config Changed: {:#?}", app.config());
                            }
                            MainEvent::LowMemory => {}

                            MainEvent::Destroy => quit = true,
                            _ => { /* ... */ }
                        }
                    }
                    _ => {}
                }

                if redraw_pending {
                    if let Some(native_window) = &native_window {
                        redraw_pending = false;

                        // Handle input, via a lending iterator
                        match app.input_events_iter() {
                            Ok(mut iter) => loop {
                                info!("Checking for next input event...");
                                if !iter.next(|event| {
                                    match event {
                                        InputEvent::KeyEvent(key_event) => {
                                            info!("GOT A KEY EVENT:{:?}", key_event.key_code());
                                        }
                                        InputEvent::MotionEvent(motion_event) => {
                                            println!("action = {:?}", motion_event.action());
                                            match motion_event.action() {
                                                MotionAction::Up => {
                                                    let pointer = motion_event.pointer_index();
                                                    let pointer =
                                                        motion_event.pointer_at_index(pointer);
                                                    let x = pointer.x();
                                                    let y = pointer.y();

                                                    println!("POINTER UP {x}, {y}");
                                                    if x < 200.0 && y < 200.0 {
                                                        println!("Requesting to show keyboard");
                                                        app.show_soft_input(true);
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        InputEvent::TextEvent(state) => {
                                            info!("Input Method State: {state:?}");
                                        }
                                        _ => {}
                                    }

                                    info!("Input Event: {event:?}");
                                    InputStatus::Unhandled
                                }) {
                                    info!("No more input available");
                                    break;
                                }
                            },
                            Err(err) => {
                                log::error!("Failed to get input events iterator: {err:?}");
                            }
                        }

                        info!("Render...");
                        //let _ = ndk_context_jni_test(app.clone(), native_window);
                    }
                }
            },
        );
    }

    info!("after hello world");
    println!("after hello world");
}

/// A minimal example of how to use `ndk_context` to get a `JavaVM` + `Context and make a JNI call
fn ndk_context_jni_test(
    app: AndroidApp,
    //native_window: &ndk::native_window::NativeWindow,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get a VM for executing JNI calls
    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;

    let env = vm.attach_current_thread()?;
    let context = AndroidContentContext::from(unsafe { JObject::from_raw(ctx.context().cast()) });
    /*
    if let RawWindowHandle::AndroidNdk(ndk_handle) = native_window.raw_window_handle()? {
        let foo = ndk_handle.a_native_window;
    }
    */

    //let text = env.new_string(format!("FOOBAR")).expect("Failed to build string");

    let text_view = AndroidWidgetTextView::new_1android_widget_text_view_landroid_content_context_2(
        *env, context,
    );
    let string = env.new_string("foobar").expect("Failed to build string");




    let jchar_array = env.new_char_array(10).expect("Failed to build char array");
    let _ = env.set_char_array_region(
        jchar_array,
        0,
        &[
            ('a' as jchar).try_into().unwrap(),
            ('b' as jchar).try_into().unwrap(),
            ('c' as jchar).try_into().unwrap(),
        ],
    );
    let jchar_array = unsafe { JObject::from_raw(jchar_array) };
    let jchar_array = JavaLangCharSequence::from(jchar_array);

    /*
    text_view.set_text_ljava_lang_char_sequence_2(
        *env,
        jchar_array,
    );
    text_view.as_android_view_view().set_layout_params(
        AndroidViewViewGroupLayoutParams,
    );
    */
    text_view
        .as_android_view_view()
        .set_background_color(*env, 0x000000);


    let layout =
        AndroidWidgetLinearLayout::new_1android_widget_linear_layout_landroid_content_context_2(
            *env, context,
        );
    layout.set_orientation(*env, 1);
    layout.as_android_view_view_group().as_android_view_view().set_padding(*env, 16, 16, 16, 16);
    layout
        .as_android_view_view_group()
        .add_view_landroid_view_view_2(*env, text_view.as_android_view_view());
    let activity =
        AndroidAppActivity::from(unsafe { JObject::from_raw(app.activity_as_ptr().cast()) });
    let window = activity.get_window(*env);
    /*
    layout
        .as_android_view_view_group()
        .add_view_landroid_view_view_2ii(*env, text_view.as_android_view_view(), 200, 200);
    */

    //view_group.add_view_landroid_view_view_2(*env, text_view.as_android_view_view());
    window.set_content_view_landroid_view_view_2(
        *env,
        layout.as_android_view_view_group().as_android_view_view()
    );

    let text_editor_view =
        AndroidWidgetEditText::new_1android_widget_edit_text_landroid_content_context_2(
            *env, context,
        );

    // Since we aren't making JNI calls within the implementation of a native call from the JavaVM
    // we wrap the reference in an `AutoLocal` to make sure it will be deleted.
    let _int_ref = env.auto_local(
        env.new_object("java/lang/Integer", "(I)V", &[JValue::Int(42)])
            .unwrap(),
    );

    Ok(())
}
