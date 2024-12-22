use std::{env, path::Path, thread::current};

use gl::types::{GLfloat, GLsizei};

use super::gl_wrapper::{BufferObject, ShaderProgram, Vao, VertexAttribute};

pub fn draw_square(x: f32, y: f32, size: f32, color: [f32; 4]) {
    let half_size = size / 2.0;
    let vertices: [f32; 28] = [
        x + half_size, y + half_size, 0.0, color[0], color[1], color[2], color[3], // Top-right
        x + half_size, y - half_size, 0.0, color[0], color[1], color[2], color[3], // Bottom-right
        x - half_size, y - half_size, 0.0, color[0], color[1], color[2], color[3], // Bottom-left
        x - half_size, y + half_size, 0.0, color[0], color[1], color[2], color[3], // Top-left
    ];

    let indices = [0, 1, 3, 1, 2, 3];
    let vao = Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32data(&vertices);

    let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ibo.bind();
    ibo.store_i32data(&indices);


    let exe_dir = env::current_exe().expect("Failed to get the executable path");
    let exe_dir = exe_dir.parent().expect("Failed to get the parent directory");

    let vertex_shader_source = r#"
        #version 330 core

        layout(location = 0) in vec3 aPos; // Posição do vértice (x, y, z)
        layout(location = 1) in vec4 aColor; // Cor do vértice (r, g, b, a)

        out vec4 vColor; // Enviar cor para o Fragment Shader

        void main() {
            gl_Position = vec4(aPos, 1.0); // Converte a posição para clip-space
            vColor = aColor; // Passa a cor para o próximo estágio
        }

    "#;

    let fragment_shader_source = r#"
        #version 330 core

        in vec4 vColor; // Recebe a cor do Vertex Shader

        out vec4 FragColor; // Cor final do pixel

        void main() {
            FragColor = vColor; // Define a cor final do fragmento
        }
    "#;

    let shader_program = ShaderProgram::new(
        &vertex_shader_source,
        &fragment_shader_source,
    );
    shader_program.bind();
    let position_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        7 * std::mem::size_of::<GLfloat>() as GLsizei, // 7 componentes por vértice (x, y, z, r, g, b, a)
        std::ptr::null(),
    );
    
    position_attribute.enable();

    let color_attribute = VertexAttribute::new(
        1,
        4, // Cor: r, g, b, a
        gl::FLOAT,
        gl::FALSE,
        7 * std::mem::size_of::<GLfloat>() as GLsizei, 
        (3 * std::mem::size_of::<GLfloat>()) as *const _,
    );
    color_attribute.enable();


    unsafe {
        gl::DrawElements(
            gl::TRIANGLES,                // Modo de desenho
            indices.len() as i32,         // Número de índices
            gl::UNSIGNED_INT,             // Tipo dos índices
            std::ptr::null(),                  // Deslocamento
        );
    }

    vao.unbind();
    vbo.unbind();
    ibo.unbind();
}