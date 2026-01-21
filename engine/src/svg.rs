#[derive(Debug, Clone)]
pub struct SvgDocument {
    width: u32,
    height: u32,
    illustrator_friendly: bool,
    paths: Vec<SvgPath>,
}

impl SvgDocument {
    pub fn new(width: u32, height: u32, illustrator_friendly: bool) -> Self {
        Self {
            width,
            height,
            illustrator_friendly,
            paths: Vec::new(),
        }
    }

    pub fn add_path(&mut self, path: SvgPath) {
        self.paths.push(path);
    }

    pub fn render(&self) -> String {
        let mut svg = String::new();
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">",
            self.width, self.height, self.width, self.height
        ));
        if self.illustrator_friendly {
            svg.push_str("<g id=\"vector-layers\">");
        }
        for path in &self.paths {
            svg.push_str(&path.render());
        }
        if self.illustrator_friendly {
            svg.push_str("</g>");
        }
        svg.push_str("</svg>");
        svg
    }
}

#[derive(Debug, Clone)]
pub struct SvgPath {
    pub d: String,
    pub fill: [u8; 4],
}

impl SvgPath {
    pub fn rect(x: u32, y: u32, w: u32, h: u32, fill: [u8; 4]) -> Self {
        let d = format!(
            "M {} {} H {} V {} H {} Z",
            x,
            y,
            x + w,
            y + h,
            x
        );
        Self { d, fill }
    }

    pub fn render(&self) -> String {
        let fill = format!(
            "#{:02X}{:02X}{:02X}",
            self.fill[0], self.fill[1], self.fill[2]
        );
        format!("<path d=\"{}\" fill=\"{}\" />", self.d, fill)
    }

    pub fn node_count(&self) -> usize {
        self.d.split_whitespace().count()
    }
}
