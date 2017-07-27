extern crate gl;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;
use std::ffi::CStr;

pub static SHDR_GLYPH_RASTERIZER: &'static str = "
    #version 150\n
    in vec2 texcoord;
    uniform sampler2D tex;
    uniform vec4 color;
    out vec4 fragmentColor;
    void main() {
        fragmentColor = vec4(1, 1, 1, texture2D(tex, texcoord).r) * color;
    }
";


pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        println!("shader source len={}", src.len());

        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        println!("shader compiled");

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        println!("shader compile status read; status={}", status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            println!("shader info log len={}", len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader,
                                 len,
                                 ptr::null_mut(),
                                 buf.as_mut_ptr() as *mut GLchar);
            
            panic!("Failed compilation log output:\n{}", str::from_utf8(&buf)
                            .ok()
                            .expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

pub unsafe fn get_attrib(program: GLuint, name: *const GLchar) -> GLint {
    let attribute = gl::GetAttribLocation(program, name);
    if attribute == -1 {
        panic!("Could not bind attrib {}", CStr::from_ptr(name)
                            .to_str()
                            .ok()
                            .expect("Attrib not valid c_string"));
    }
    attribute
}

pub unsafe fn get_uniform(program: GLuint, name: *const GLchar) -> GLint {
    let uniform = gl::GetUniformLocation(program, name);
    if uniform == -1 {
        panic!("Could not bind uniform {}", CStr::from_ptr(name)
                            .to_str()
                            .ok()
                            .expect("Attrib not valid c_string"));
    }
    uniform
}