use std::ops::{Index, IndexMut};

pub struct Image<PixelType> {
    data: [[PixelType; 28]; 28],
}
impl<PixelType> Index<(usize, usize)> for Image<PixelType> {
    type Output = PixelType;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}
impl<PixelType> IndexMut<(usize, usize)> for Image<PixelType> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}
