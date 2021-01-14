use super::Frame;


pub struct RGBAImage {
    pub width:u32,
    pub height:u32,
    pub pixels:Vec<u8>
}

impl RGBAImage {
    pub fn flip(self) -> Self
    {
        let mut pixels = self.pixels;
        let len = pixels.len() / 2;
        let (top, bottom) = pixels.split_at_mut(len);
        assert!(top.len() == bottom.len());

        let width = self.width as usize;
        let height = self.height as usize / 2 as usize;
        let row_size = width * 4 as usize;
        let mut temp = vec![0; row_size];

        for row1 in 0..height {
            let row2 = height - row1 -1;
            let row1 = row1 * row_size .. (row1 + 1) * row_size;
            let row2 = row2 * row_size .. (row2 + 1) * row_size;
            
            // copy row1 to temp
            temp.copy_from_slice(&top[row1.clone()]);

            // copy row2 to row1
            top[row1.clone()].copy_from_slice(&bottom[row2.clone()]);

            // copy temp to row2
            bottom[row2.clone()].copy_from_slice(&temp);
           
        }



        RGBAImage {
            width:self.width,
            height:self.height,
            pixels:pixels
        }
    }

    /// Constructs a `Frame` with normalized coordinates based upon dimensions of `RGBAImage`
    pub fn frame(&self, x:u32, y:u32, width:u32, height:u32) -> Frame {
        Frame {
            u:x as f32 / 1.0,
            v:y as f32 / 1.0,
            w: width as f32 / 1.0,
            h: height as f32 / 1.0 
        }
    }
}
