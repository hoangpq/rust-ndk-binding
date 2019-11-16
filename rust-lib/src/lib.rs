/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JString, JClass, JObject};
    use self::jni::sys::jstring;
    use std::ffi::CString;
    use std::os::raw::c_void;

    mod fractal;
    mod graphic;

    #[no_mangle]
    pub unsafe extern fn Java_com_example_myktapp_MainActivity_greeting(env: JNIEnv, _: JClass) -> jstring {
        let world_ptr = CString::new("Hello world from Rust world").unwrap();
        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_myktapp_MainActivity_renderFractal(env: JNIEnv, _: JClass, bmp: JObject) {
        let mut info = graphic::AndroidBitmapInfo::new();
        let raw_env = env.get_native_interface();

        let bmp = bmp.into_inner();

        // Read bitmap info
        graphic::bitmap_get_info(raw_env, bmp, &mut info);
        let mut pixels = 0 as *mut c_void;

        // Lock pixel for draw
        graphic::bitmap_lock_pixels(raw_env, bmp, &mut pixels);

        let pixels =
            std::slice::from_raw_parts_mut(pixels as *mut u8, (info.stride * info.height) as usize);

        fractal::render(pixels, info.width as u32, info.height as u32);
        graphic::bitmap_unlock_pixels(raw_env, bmp);
    }
}
