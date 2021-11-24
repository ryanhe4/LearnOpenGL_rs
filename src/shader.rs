#![allow(non_snake_case)]
use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use cgmath::{Matrix, Matrix4};
use gl;
use gl::types::*;

pub struct Shader {
    pub ID: u32,
}

#[derive(Debug)]
enum ShaderType {
    VERTEX,
    FRAGMENT,
    PROGRAM,
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Shader {
    pub fn new(vertexPath: &str, fragmentPath: &str) -> Shader {
        let mut shader = Shader { ID: 0 };
        let mut vShaderFile =
            File::open(vertexPath).unwrap_or_else(|_| panic!("Failed to open {}", vertexPath));
        let mut fShaderFile =
            File::open(fragmentPath).unwrap_or_else(|_| panic!("Failed to open {}", fragmentPath));
        let mut vertexCode = String::new();
        let mut fragmentCode = String::new();
        vShaderFile
            .read_to_string(&mut vertexCode)
            .expect("Failed to read vertex shader");
        fShaderFile
            .read_to_string(&mut fragmentCode)
            .expect("Failed to read fragment shader");
        let vShaderCode = CString::new(vertexCode.as_bytes()).unwrap();
        let fShaderCode = CString::new(fragmentCode.as_bytes()).unwrap();

        // compile shader
        unsafe {
            //v shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.checkCompileErrors(vertex, ShaderType::VERTEX);
            //f shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.checkCompileErrors(fragment, ShaderType::FRAGMENT);
            // shader program
            let ID = gl::CreateProgram();
            gl::AttachShader(ID, vertex);
            gl::AttachShader(ID, fragment);
            gl::LinkProgram(ID);
            shader.checkCompileErrors(ID, ShaderType::PROGRAM);
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.ID = ID;
        }

        shader
    }

    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn useProgram(&self) {
        gl::UseProgram(self.ID)
    }

    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn setBool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.ID, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setInt(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.ID, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setFloat(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.ID, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    // pub unsafe fn setVector3(&self, name: &CStr, value: &Vector3<f32>) {
    //     gl::Uniform3fv(
    //         gl::GetUniformLocation(self.ID, name.as_ptr()),
    //         1,
    //         value.as_ptr(),
    //     );
    // }
    /// ------------------------------------------------------------------------
    // pub unsafe fn setVec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
    //     gl::Uniform3f(gl::GetUniformLocation(self.ID, name.as_ptr()), x, y, z);
    // }
    // /// ------------------------------------------------------------------------
    pub unsafe fn setMat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.ID, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }
    unsafe fn checkCompileErrors(&self, shader: u32, type_: ShaderType) {
        let mut success = gl::FALSE as GLint;
        let mut infoLog = Vec::with_capacity(1024);
        infoLog.set_len(1024 - 1);
        match type_ {
            ShaderType::VERTEX | ShaderType::FRAGMENT => {
                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                if success != gl::TRUE as GLint {
                    gl::GetShaderInfoLog(
                        shader,
                        1024,
                        ptr::null_mut(),
                        infoLog.as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                        "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                        type_,
                        str::from_utf8(&infoLog).unwrap()
                    );
                }
            }
            ShaderType::PROGRAM => {
                gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
                if success != gl::TRUE as GLint {
                    gl::GetProgramInfoLog(
                        shader,
                        1024,
                        ptr::null_mut(),
                        infoLog.as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                        "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                        type_,
                        str::from_utf8(&infoLog).unwrap()
                    );
                }
            }
            _ => println!("checkCompileErrors::UNKNOWN TYPE!!"),
        }
    }
}
