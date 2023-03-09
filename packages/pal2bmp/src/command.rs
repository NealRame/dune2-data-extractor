use std::error::Error;

use std::fs;
use std::io;

use crate::config::Config;

fn read_color<T>(reader: &mut T)
    -> Option<bitmap::Color>
    where T: io::Read
{
    let mut buf = [0; 3];
    match reader.read(&mut buf) {
        Ok(3) => {
            let (red, green, blue) = (buf[0], buf[1], buf[2]);
            Some(bitmap::Color::new(4*red, 4*green, 4*blue))
        },
        _ => None,
    }
}

struct PaletteColorReader<T>
    where T: io::Read {
    source: T,
}

impl<T> PaletteColorReader<T>
    where T: io::Read {
    fn new(source: T) -> PaletteColorReader<T> {
        PaletteColorReader { source }
    }
}

impl<T> Iterator for PaletteColorReader<T>
    where T: io::Read {
    type Item = bitmap::Color;

    fn next(&mut self) -> Option<Self::Item> {
        read_color(&mut self.source)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = config.output_filepath.parent() {
        fs::create_dir_all(parent)?;
    }

    println!(" input: {:?}", config.input_filepath);

    let palette = PaletteColorReader::new(fs::File::open(&config.input_filepath)?).collect::<Vec<bitmap::Color>>();

    let palette_width = 32*16 as u32;
    let palette_height = 32*((palette.len() as f32)/16.).ceil() as u32;
    
    let mut palette_bitmap = bitmap::Bitmap::new(palette_width, palette_height);
    
    println!("palette: ({},{}) = {} colors", palette_width, palette_height, palette.len());

    for (i, color) in palette.iter().enumerate() {
        let x = 32*(i%16) as u32;
        let y = 32*(i/16) as u32;

        println!("{} - ({},{}) = {:?}", i, x, y, color);

        let rect = bitmap::Rect::from_point_and_size(
            bitmap::Point { x, y },
            bitmap::Size { width: 32, height: 32 }
        );

        palette_bitmap.fill_rect(&rect, *color);
    }

    let mut output = fs::File::create(&config.output_filepath)?;

    palette_bitmap.write(&mut output)?;

    // for (i, color) in palette.iter().enumerate() {
    //     println!("{} - {:?}", i, color);
    // }

    // println!("output: {:?}", config.output_filepath);

    Ok(())
}