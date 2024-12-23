use std::{
    io::Read,
    ops::{Index, IndexMut},
};

pub struct Image<PixelType> {
    pub data: [[PixelType; 28]; 28],
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
pub struct ImageSet<PixelType, LabelType> {
    pub data: Vec<(Image<PixelType>, LabelType)>,
}
impl<PixelType, LabelType> Index<usize> for ImageSet<PixelType, LabelType> {
    type Output = (Image<PixelType>, LabelType);
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<PixelType, LabelType> IndexMut<usize> for ImageSet<PixelType, LabelType> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub fn load_file_u8<T: Read, U: Read>(labels: T, images: U) -> ImageSet<u8, u8> {
    let images = images_from_file_u8(images);
    let labels = labels_from_file_u8(labels);
    let data: Vec<_> = images.into_iter().zip(labels.into_iter()).collect();
    ImageSet { data }
}
fn images_from_file_u8<T: Read>(file: T) -> Vec<Image<u8>> {
    let mut data: Vec<u8> = file.bytes().flatten().collect();
    assert_eq!(&[0u8, 0], &data[0..2]);
    assert_eq!(data[2], 0x08);
    let dims = data[3];
    let data_reader = &mut data[4..].iter_mut();
    let mut dimension_sizes = Vec::new();
    for _ in 0..dims {
        if let (Some(byte0), Some(byte1), Some(byte2), Some(byte3)) = (
            data_reader.next(),
            data_reader.next(),
            data_reader.next(),
            data_reader.next(),
        ) {
            let size = u32::from_be_bytes([*byte0, *byte1, *byte2, *byte3]);
            dimension_sizes.push(size);
        } else {
            panic!("Not enough data in dimension list")
        }
    }
    let mut data = Vec::new();
    for image_index in 0..dimension_sizes[0] {
        let mut image = [[0u8; 28]; 28];
        for row_index in 0..dimension_sizes[1] {
            let mut row = [0u8; 28];
            for pixel_index in 0..dimension_sizes[2] {
                row[pixel_index as usize] = *data_reader.next().unwrap();
            }
            image[row_index as usize] = row;
        }
        let img = Image { data: image };
        data.push(img);
    }
    assert_eq!(dimension_sizes.len(), 3);
    data
}
fn labels_from_file_u8<T: Read>(file: T) -> Vec<u8> {
    let mut data: Vec<u8> = file.bytes().flatten().collect();
    assert_eq!(&[0u8, 0], &data[0..2]);
    assert_eq!(data[2], 0x08);
    let dims = data[3];
    let data_reader = &mut data[4..].iter_mut();
    let mut dimension_sizes = Vec::new();
    for _ in 0..dims {
        if let (Some(byte0), Some(byte1), Some(byte2), Some(byte3)) = (
            data_reader.next(),
            data_reader.next(),
            data_reader.next(),
            data_reader.next(),
        ) {
            let size = u32::from_be_bytes([*byte0, *byte1, *byte2, *byte3]);
            dimension_sizes.push(size);
        } else {
            panic!("Not enough data in dimension list")
        }
    }
    assert_eq!(dimension_sizes.len(), 1);
    let mut data = Vec::new();
    for item in data_reader {
        data.push(*item);
    }
    data
}
