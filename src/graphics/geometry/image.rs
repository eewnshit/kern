use gl::types::{GLfloat, GLsizei};
use crate::graphics::gl_wrapper::{BufferObject, ShaderProgram, Vao, VertexAttribute};
use image::GenericImageView;

pub struct KImage {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub texture_id: u32,
    vao: Vao,
    vbo: BufferObject,
    ibo: BufferObject,
    shader_program: ShaderProgram,
}

impl KImage {
    pub fn new(x: f32, y: f32, width: f32, height: f32, image_path: &str) -> Self {
        let img = image::open(image_path).expect("Error on loading image");
        img.flipv();
        let (img_width, img_height) = img.dimensions();
        let img_data = img.to_rgba8();
        let img_data = img_data.into_raw();

        let mut texture_id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img_width as i32,
                img_height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img_data.as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        let vertices: [f32; 20] = [
            x + width / 2.0, y + height / 2.0, 0.0, 1.0, 0.0,
            x + width / 2.0, y - height / 2.0, 0.0, 1.0, 1.0,
            x - width / 2.0, y - height / 2.0, 0.0, 0.0, 1.0,
            x - width / 2.0, y + height / 2.0, 0.0, 0.0, 0.0, 
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
            layout(location = 1) in vec2 aTexCoord;
            out vec2 TexCoord;
            void main() {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
                TexCoord = aTexCoord;
            }
        "#;

        let fragment_shader_source = r#"
            #version 330 core
            in vec2 TexCoord;
            out vec4 FragColor;
            uniform sampler2D texture1;
            void main() {
                FragColor = texture(texture1, TexCoord);
            }
        "#;

        let shader_program = ShaderProgram::new(vertex_shader_source, fragment_shader_source);
        shader_program.bind();

        let position_attribute = VertexAttribute::new(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * std::mem::size_of::<GLfloat>() as GLsizei,
            std::ptr::null(),
        );
        position_attribute.enable();

        let texcoord_attribute = VertexAttribute::new(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * std::mem::size_of::<GLfloat>() as GLsizei,
            (3 * std::mem::size_of::<GLfloat>()) as *const _,
        );
        texcoord_attribute.enable();

        vao.unbind();
        vbo.unbind();
        ibo.unbind();

        Self {
            x,
            y,
            width,
            height,
            texture_id,
            vao,
            vbo,
            ibo,
            shader_program,
        }
    }

    pub fn draw(&mut self) {
        self.shader_program.bind();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        
        }

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
