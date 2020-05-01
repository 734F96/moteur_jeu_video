/// Owns the rendering parameters.
#[derive(Debug)]
pub struct Params {
    pub parameters: glium::draw_parameters::DrawParameters<'static>,
}

impl Params {
    /// Returns some default parameters.
    pub fn new() -> Self {
        Self {
            parameters: glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
		backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                ..Default::default()
            },
        }
    }

    /**
    Draw only the lines of the edges of the traingles which composed ours polygons.
     **/
    pub fn polygon_line(mut self) -> Self {
        self.parameters.polygon_mode = glium::draw_parameters::PolygonMode::Line;
        return self;
    }

    /**draw only the traingles'points which composed ours polygons.**/
    pub fn polygon_point(mut self) -> Self {
        self.parameters.polygon_mode = glium::draw_parameters::PolygonMode::Point;
        return self;
    }

    /**draw all the content of ours polygons.**/
    pub fn polygon_fill(mut self) -> Self {
        self.parameters.polygon_mode = glium::draw_parameters::PolygonMode::Fill;
        return self;
    }

    /**color all the polygons with the color passed in arguments*/
    pub fn color_all(mut self, r: bool, g: bool, b: bool, a: bool) -> Self {
        self.parameters.color_mask = (r, g, b, a);
        return self;
    }

    /**enable or disable the transparency**/
    pub fn with_transparency(mut self, activated: bool) -> Self {
        if activated {
            self.parameters.blend = glium::Blend::alpha_blending();
        } else {
            self.parameters.blend = glium::Blend::default();
        }
        return self;
    }

    /// Put the drawn object in foreground (usefull for UIs)
    pub fn always_top(mut self) -> Self {
        self.parameters.depth.test = glium::DepthTest::Overwrite;
        self
    }
}
