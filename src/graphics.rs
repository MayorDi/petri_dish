use std::ffi::CString;
use gl::types::*;

pub unsafe fn compile_shader(src: String, type_shader: GLenum) -> GLuint {
    let vfshader: GLuint = gl::CreateShader(type_shader);
    let c_str = CString::new(src.as_bytes()).unwrap();

    gl::ShaderSource(vfshader, 1, &c_str.as_ptr(), std::ptr::null());
    gl::CompileShader(vfshader);

    shader_status(vfshader);

    vfshader
}

pub unsafe fn shader_status(shader: GLuint) {
    let mut status = gl::FALSE as GLint;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

    if status != (gl::TRUE as i32) {
        let mut len: GLsizei = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

        let mut buf: Vec<u8> = Vec::with_capacity(len as usize);

        gl::GetShaderInfoLog(
            shader,
            len,
            std::ptr::null_mut(),
            buf.as_mut_ptr() as *mut i8,
        );

        panic!("{} bytes\n{}", len, String::from_utf8(buf).unwrap());
    }
}

pub unsafe fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    
    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

        let mut buf = Vec::with_capacity(len as usize);
        buf.set_len((len as usize) - 1);

        gl::GetProgramInfoLog(
            program,
            len,
            std::ptr::null_mut(),
            buf.as_mut_ptr() as *mut GLchar,
        );

        panic!("{} bytes\n{}", len, String::from_utf8(buf).unwrap());
    }

    program
}

#[derive(derive_new::new, Debug, Clone, Copy)]
pub struct Vertex(pub f32, pub f32);

pub struct VertexBufferObject(pub Vec<Vertex>);