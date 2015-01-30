/*pub fn render(&self, gl: &mut Gl, ctx: &Context) {
    match self.material {
        Material::Air => {},
        Material::Land => {
            let rect = [
                (self.x * BLOCK_SIZE) as f64,
                (self.z * BLOCK_SIZE) as f64,
                BLOCK_SIZE as f64,
                BLOCK_SIZE as f64
            ];
            Rectangle::new([0.0, 0.0, 0.0, 1.0])
                .draw(rect, ctx, gl);
        }
    }
}*/
