pub struct Image {
    bytes: Vec<u8>,
    width: usize,
    height: usize,
}

pub enum ImageError {
    UncaughtException,
    TooManyBytes,
    NotEnoughBytes
}

impl Image {
    pub fn new(bytes: Vec<u8>, width: usize, height: usize) -> Result<Self, ImageError> {
        return match width*height*4 {
            d if d > bytes.len() => Err(ImageError::NotEnoughBytes),
            d if d < bytes.len() => Err(ImageError::TooManyBytes),
            d if d == bytes.len() => {
                Ok(Self {
                    bytes: bytes,
                    width: width,
                    height: height,
                })
            }
            _ => Err(ImageError::UncaughtException),
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&[u8]> {
        let i = x*4 + y*self.width*4;
        if i+4 > self.bytes.len() {
            return None
        }

        Some(&self.bytes[i..i+4])
    }

    pub fn row(&self, y: usize) -> Option<&[u8]> {
        if y >= self.height {
            return None
        }
        
        Some(&self.bytes[y*self.width*4..(y+1)*self.width*4])
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}