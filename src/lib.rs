pub mod types{
    pub enum Color {
        RGBA(u8,u8,u8,u8),
        HEX(u8,u8,u8,u8)
    }

    pub struct OBJ {
        points: Vec<f32>
    }
}

