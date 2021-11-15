#![allow(non_upper_case_globals)]

mod shader;

extern crate glfw;

use self::glfw::{Action, Context, Key};

extern crate gl;

use self::gl::types::*;

use crate::shader::Shader;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;
use std::sync::mpsc::Receiver;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const vertexShaderSource: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    
    out vec3 ourColor;
    
    void main() {
       gl_Position = vec4(aPos, 1.0);
       ourColor = aColor;
    }
"#;

const fragmentShaderSource: &str = r#"
    #version 330 core
    out vec4 FragColor;
    in vec3 ourColor;
    void main() {
       FragColor = vec4(ourColor, 1.0);
    }
"#;

#[allow(non_snake_case)]
pub fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GFLW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shaderProgram, VAO) = unsafe {
        // build and compile our shader program
        // ------------------------------------
        // vertex shader
        let shaderProgram = Shader::new("src/shaders/base.vs", "src/shaders/base.fs");

        // gl::DeleteShader(vertexShader);
        // gl::DeleteShader(fragmentShader);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertice1: [f32; 18] = [
            // first triangle
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // left
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // right
            0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
        ];

        let (mut VBO, mut VAO) = (0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO); // Memory Allocate
                                     // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertice1.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertice1[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        // gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shaderProgram, VAO)
    };

    // render loop
    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our first triangle
            // gl::UseProgram(shaderProgram);

            // let timeValue = glfw.get_time() as f32;
            // let greenValue = timeValue.sin() / 2.0 + 0.5;
            // let ourColor = CString::new("ourColor").unwrap();
            // let vertextColorLocation = gl::GetUniformLocation(shaderProgram, ourColor.as_ptr());
            // gl::Uniform4f(vertextColorLocation, 0.0, greenValue, 0.0, 1.0);
            shaderProgram.useProgram();
            gl::BindVertexArray(VAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
