#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

//#[derive(Default)]
#[derive(Debug)]
struct Color {
    red: u8,
    blue: u8,
    green: u8,
}

impl Default for Color {
    fn default() -> Color {
        Color { red: 60, blue: 60, green: 60 }
   }
}

#[derive(Debug)]
pub struct Rectangle {
    start_point: Point,
    height: f32,
    width: f32,
    color: Color,
    text: String,
}

impl Rectangle
{
    pub fn new(x: f32, y: f32, height: f32, width: f32, title: String) -> Self
    {
        let start_point: Point = Point { x: x, y: y };

        Rectangle {
            start_point: start_point,
            height: height,
            width: width,
            color: Color::default(),
            text: title,
        }
    }
}

/*
System = Game
display: l'affichage
imgui context = frame
platform rien à foutre

*/