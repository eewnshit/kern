use gl::types::{GLfloat, GLsizei};
use crate::graphics::gl_wrapper::{BufferObject, ShaderProgram, Vao, VertexAttribute};

pub struct KLine {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub color: [f32; 4],
    vao: Vao,
    vbo: BufferObject,
    ibo: BufferObject,
    shader_program: ShaderProgram,
}

impl KLine {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, color: [f32; 4]) -> Self {
        let vertices: [f32; 14] = [
            x1, y1, 0.0, color[0], color[1], color[2], color[3], // Ponto 1
            x2, y2, 0.0, color[0], color[1], color[2], color[3], // Ponto 2
        ];

        let indices = [0, 1];

        let vao = Vao::new();
        vao.bind();

        let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        vbo.bind();
        vbo.store_f32data(&vertices);

        let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        ibo.bind();
        ibo.store_i32data(&indices);

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

        // Criação do programa de shader
        let vertex_shader_source = r#"
            #version 330 core
            layout(location = 0) in vec3 aPos;
            layout(location = 1) in vec4 aColor;
            out vec4 vColor;
            void main() {
                gl_Position = vec4(aPos, 1.0);
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

        let shader_program = ShaderProgram::new(&vertex_shader_source, &fragment_shader_source);

        Self {
            x1,
            y1,
            x2,
            y2,
            color,
            vao,
            vbo,
            ibo,
            shader_program,
        }
    }

    pub fn draw(&self) {
        self.shader_program.bind();
        self.vao.bind();
        unsafe {
            gl::DrawElements(
                gl::LINES,
                2, 
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
        self.vao.unbind();
    }

    pub fn is_colliding(&self) {
    }
}
