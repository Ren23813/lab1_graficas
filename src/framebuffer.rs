use raylib::prelude::*;

pub struct Framebuffer{
    pub width:i32,
    pub height:i32,
    pub color_buffer: Image,
    background_color:Color,
    current_color:Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color:Color) -> Self{
        let color_buffer = Image::gen_image_color(width,height,background_color);
        Framebuffer{
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self){
        self.color_buffer = Image::gen_image_color(self.width,self.height,self.background_color)
    }

    pub fn set_pixel(&mut self, x:i32, y:i32){
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color:Color){
        self.background_color = color;
        self.clear();
    }

    pub fn set_current_color(&mut self, color:Color){
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path:&str){
        Image::export_image(&self.color_buffer, file_path);
    }

}


//esto se puede poner en main para probar únicamente el funcionamiento del framebuffer (por si lo quiere probar, dennis/aux :D)
//   let (mut rl,_thread) = raylib::init()
//         .size(800, 600)
//         .title("Framebuffer Example")
//         .build();

//     // Crear un framebuffer de 800x600 con fondo negro
//     let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);

//         // Limpiar y establecer un nuevo fondo azul
//     framebuffer.set_background_color(Color::BLUE);
    
//     // Cambiar el color actual a rojo
//     framebuffer.set_current_color(Color::WHITE);

//     // Dibujar algunos píxeles
//     framebuffer.set_pixel(400, 300);
//     framebuffer.set_pixel(401, 300);
//     framebuffer.set_pixel(400, 301);

//     // Renderizar el framebuffer a un archivo
//     framebuffer.render_to_file("output.png");
