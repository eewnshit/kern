use core::panic;
use std::{collections::HashMap, ffi::CString, fs::File, io::Read, mem, os::raw::c_void, ptr};

use cgmath::{Matrix, Matrix4};
use gl::types::{GLboolean, GLchar, GLenum, GLint, GLsizei, GLuint};

pub struct Vao {
    id: gl::types::GLuint,
}

impl Vao {
    pub fn new() -> Vao {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Vao {id}
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct BufferObject {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum
}

impl BufferObject {
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> BufferObject {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        BufferObject { id, r#type, usage }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }
    pub fn store_f32data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as * const f32 as *const c_void,
                self.usage
            )
        }
    }
    pub fn store_i32data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as * const i32 as *const c_void,
                self.usage
            )
        }
    }
}


pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {

    pub fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void
    ) -> VertexAttribute {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }
        VertexAttribute {index}
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }
    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

pub struct ShaderProgram {
    program_handle: u32,
    uniform_ids: HashMap<String, GLint>
}

impl ShaderProgram {
    // Modificado para aceitar o código dos shaders como strings
    pub fn new(vertex_shader_source: &str, fragment_shader_source: &str) -> ShaderProgram {
        unsafe {
            // Compilando o vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            Self::check_shader_compile(vertex_shader);

            // Compilando o fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            Self::check_shader_compile(fragment_shader);

            // Criando o programa e associando os shaders
            let program_handle = gl::CreateProgram();
            gl::AttachShader(program_handle, vertex_shader);
            gl::AttachShader(program_handle, fragment_shader);
            gl::LinkProgram(program_handle);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            ShaderProgram {
                program_handle,
                uniform_ids: HashMap::new(),
            }
        }
    }

    // Função para verificar se o shader compilou corretamente
    fn check_shader_compile(shader: u32) {
        unsafe {
            let mut success: i32 = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut log_len: i32 = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);
                let mut log: Vec<u8> = Vec::with_capacity(log_len as usize);
                gl::GetShaderInfoLog(
                    shader,
                    log_len,
                    &mut log_len,
                    log.as_mut_ptr() as *mut GLchar,
                );
                log.set_len(log_len as usize);
                panic!("Shader compilation failed: {}", String::from_utf8_lossy(&log));
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_handle);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn create_uniform(&mut self, uniform_name: &str) {
        let uniform_location = unsafe {
            gl::GetUniformLocation(
                self.program_handle,
                CString::new(uniform_name).unwrap().as_ptr(),
            )
        };
        if uniform_location < 0 {
            panic!("Cannot locate uniform: {}", uniform_name);
        } else {
            self.uniform_ids
                .insert(uniform_name.to_string(), uniform_location);
        }
    }

    pub fn set_matrix4fv_uniform(&self, uniform_name: &str, matrix: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_ids[uniform_name],
                1,
                gl::FALSE,
                matrix.as_ptr(),
            );
        }
    }
}