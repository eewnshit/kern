
use gl::types::{GLfloat, GLsizei};

use crate::graphics::gl_wrapper::{BufferObject, ShaderProgram, Vao, VertexAttribute};

pub struct KSquare {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: [f32; 4],
}


impl KSquare {

    pub fn new(x: f32, y: f32, size: f32, color: [f32; 4]) -> Self {
        let new_one = Self {
            x,
            y,
            size,
            color,
        };



        new_one
    }

    pub fn draw(&self) {
        let half_size = self.size / 2.0;
        let vertices: [f32; 28] = [
            self.x + half_size, self.y + half_size, 0.0, self.color[0], self.color[1], self.color[2], self.color[3], // Top-right
            self.x + half_size, self.y - half_size, 0.0, self.color[0], self.color[1], self.color[2], self.color[3], // Bottom-right
            self.x - half_size, self.y - half_size, 0.0, self.color[0], self.color[1], self.color[2], self.color[3], // Bottom-left
            self.x - half_size, self.y + half_size, 0.0, self.color[0], self.color[1], self.color[2], self.color[3], // Top-left
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
    
    
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,           
                indices.len() as i32,       
                gl::UNSIGNED_INT,         
                std::ptr::null(),           
            );
        }
    
        vao.unbind();
        vbo.unbind();
        ibo.unbind();
    }

    pub fn is_colliding(&self) {

    }
}
