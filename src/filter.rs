use {
  image::{ImageBuffer, Pixel},
  num::Num,
};

pub type Image<P> = ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>;

pub struct Kernel<'a, K> {
  data: &'a [K],
  width: u32,
  height: u32,
}

impl<'a, K: Num + Copy + 'a> Kernel<'a, K> {
  pub fn filter<P, F, Q: Pixel>(&self, image: &Image<P>, mut filter: F)
  where
    P: Pixel<Subpixel: Into<K>>,
    F: FnMut(&mut Q::Subpixel, K),
  {
    let (width, height) = image.dimensions();
    let mut out = Image::<Q>::new(width, height);

    let (k_width, k_height) = (self.width as i64, self.height as i64);
    let (width, height) = (width as i64, height as i64);

    let mut acc: Vec<K> = vec![num::zero(); P::CHANNEL_COUNT as usize];

    for y in 0..height {
      for x in 0..width {}
    }
  }
}
