use wasm_bindgen::Clamped;
use yew::web_sys::ImageData;

pub struct Screen {
    pub bytes: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let size = 4 * width * height;
        let mut bytes = Vec::with_capacity(4 * width * height);
        bytes.resize(size, 0);
        Self {
            bytes,
            width,
            height,
        }
    }

    pub fn put_pixel(&mut self, w: usize, h: usize, v: u8) {
        let offset = 4 * (self.width * h + w);
        self.bytes[offset] = v;
        self.bytes[offset + 1] = v;
        self.bytes[offset + 2] = v;
        self.bytes[offset + 3] = 255;
    }

    pub fn image_data(&self) -> ImageData {
        let clamped_bytes = Clamped(self.bytes.as_slice());
        ImageData::new_with_u8_clamped_array(clamped_bytes, self.width as u32).unwrap()
    }
}

fn repeat(a: f64, b: f64, limit: usize) -> usize {
    let mut x0 = 0.;
    let mut y0 = 0.;
    for k in 0..limit {
        let x = x0 * x0 - y0 * y0 + a;
        let y = 2. * x0 * y0 + b;
        if x * x + y * y >= 4. {
            return k;
        }
        x0 = x;
        y0 = y;
    }
    return limit;
}

pub fn mandelbrot_screen(
    width: usize,
    height: usize,
    x0: f64,
    y0: f64,
    d: f64,
    limit: usize,
) -> Screen {
    let mut screen = Screen::new(width, height);
    for h in 0..screen.height {
        let dy = y0 + d * (h as f64);
        for w in 0..screen.width {
            let dx = x0 + d * (w as f64);
            let k = repeat(dx, dy, limit);
            let v = (k * 255 / limit) as u8;
            screen.put_pixel(w, h, v);
        }
    }
    screen
}
