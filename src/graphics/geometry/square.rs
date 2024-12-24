
use gl::types::{GLfloat, GLsizei};

use crate::graphics::gl_wrapper::{BufferObject, ShaderProgram, Vao, VertexAttribute};

pub struct KSquare {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: [f32; 4],
    vao: Vao,
    vbo: BufferObject,
    ibo: BufferObject,
    shader_program: ShaderProgram,
}

impl KSquare {
    pub fn new(x: f32, y: f32, size: f32, color: [f32; 4]) -> Self {
        let half_size = size / 2.0;
        let vertices: [f32; 28] = [
            x + half_size, y + half_size, 0.0, color[0], color[1], color[2], color[3],
            x + half_size, y - half_size, 0.0, color[0], color[1], color[2], color[3],
            x - half_size, y - half_size, 0.0, color[0], color[1], color[2], color[3],
            x - half_size, y + half_size, 0.0, color[0], color[1], color[2], color[3],
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

        let vertex_shader_source = r#"
            #version 330 core
            layout(location = 0) in vec3 aPos;
            layout(location = 1) in vec4 aColor;
            out vec4 vColor;
            void main() {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
                vColor = aColor;
            }
        "#;

        let fragment_shader_source = r#"
            #version 330 core
            in vec4 vColor;
            out vec4 FragColor;
            void main() {
                FragColor = vColor;
            }
        "#;

        let shader_program = ShaderProgram::new(vertex_shader_source, fragment_shader_source);
        shader_program.bind();

        let position_attribute = VertexAttribute::new(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            7 * std::mem::size_of::<GLfloat>() as GLsizei,
            std::ptr::null(),
        );
        position_attribute.enable();

        let color_attribute = VertexAttribute::new(
            1,
            4,
            gl::FLOAT,
            gl::FALSE,
            7 * std::mem::size_of::<GLfloat>() as GLsizei,
            (3 * std::mem::size_of::<GLfloat>()) as *const _,
        );
        color_attribute.enable();

        vao.unbind();
        vbo.unbind();
        ibo.unbind();

        Self {
            x,
            y,
            size,
            color,
            vao,
            vbo,
            ibo,
            shader_program,
        }
    }

    pub fn draw(&mut self) {
        // Atualiza os vértices com as novas posições
        let half_size = self.size / 2.0;
        let vertices: [f32; 28] = [
            self.x + half_size, self.y + half_size, 0.0, self.color[0], self.color[1], self.color[2], self.color[3],
            self.x + half_size, self.y - half_size, 0.0, self.color[0], self.color[1], self.color[2], self.color[3],
            self.x - half_size, self.y - half_size, 0.0, self.color[0], self.color[1], self.color[2], self.color[3],
            self.x - half_size, self.y + half_size, 0.0, self.color[0], self.color[1], self.color[2], self.color[3],
        ];
    
        // Atualiza o VBO com os novos dados de vértices
        self.vbo.bind();
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                std::mem::size_of_val(&vertices) as isize,
                vertices.as_ptr() as *const _,
            );
        }
    
        // Desenha o quadrado
        self.shader_program.bind();
        self.vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
        self.vao.unbind();
    }
}
