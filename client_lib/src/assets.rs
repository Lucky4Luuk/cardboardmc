pub trait Texture {
    fn write_asset_rgba(&mut self, data_res: (u32, u32), data: Vec<u8>);
}
