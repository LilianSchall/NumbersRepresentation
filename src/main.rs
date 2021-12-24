extern crate sdl2; 

use sdl2::pixels::Color;

//for form

use sdl2::rect::Rect;
use sdl2::rect::Point;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::render::WindowCanvas;

use std::path::Path;


//is_prime function: tests if a natural number is prime.
//
// @param value: the number to test
// @return boolean: is the value a prime number ?
fn is_prime (value: u32) -> bool {
    if value == 1 || value % 2 == 0
    {
        return value == 2;
    }
    let mut buffer: u32 = 3;

    while buffer * buffer < value
    {
        if value % buffer == 0
        {
            return false;
        }
        buffer += 2;
    }
    true
}

// PNumber structure: representation of a number that is used in order to determine its
// position in a polar base.
// @state value: the number to represent
// @state x: the x offset from the center
// @state y: the y offset from the center
// @state dx: the x distance between (0,0) and the point representing the number
// @state dy: the y distance between (0,0) and the point representing the number
// @state prime: is the number prime ?
struct PNumber {
    value: u32,
    x: f64,
    y: f64,
    dx: i32,
    dy: i32,
    prime: bool
}

// PNumber has one method and an initializer.
impl PNumber {

    //Constructor: creates a PNumber instance.
    fn create (x: f64, y: f64, value: u32, dx: i32, dy: i32) -> PNumber {
        PNumber {
            value,
            x,
            y,
            dx,
            dy,
            prime: is_prime(value)
        }
    }

    // Update method: update the new dx and dy states of a PNumber instance
    // based on a coefficient.
    // @param center: the point representing the center of the graph.
    // @param distance_coeff: the coefficient on which we base our dx and dy update.
    // @return boolean: is the point on screen ?
    fn update(&mut self, center: &Point, distance_coeff: f64) -> bool
    {
        self.dx = center.x + ((distance_coeff * self.x) as i32);
        self.dy = center.y - ((distance_coeff * self.y) as i32);

        return ! (self.dx > center.x * 2 || self.dy > center.y * 2 || self.dx < 0 || self.dy < 0) 
    }
}

// render function: prints a new frame to the user.
// @param canvas: the new frame to print.
fn render(canvas: &mut WindowCanvas) {
    canvas.present();
}

// calculate_xvalue function: calculate the x distance from center of a PNumber
// @param value: the number to represent
// @return float 64 bits: the x distance from the center
fn calculate_xvalue(value: f64) -> f64 {
    let x: f64 = value * value.cos();

    x
}

// calculate_yvalue function: calculate the y distance from center of a PNumber
// @param value: the number to represent
// @return float 64 bits: the y distance from the center
fn calculate_yvalue(value: f64) -> f64 {
    let z: f64 = value * value.sin();

    z
}


// add_number function: add a new PNumber instance to the list of numbers to print on the graph
// if the number we want to represent is offscreen, we do not add it to the list until it isn't
// offscreen anymore
// @param numbers: the list of PNumber instances
// @param value: the new number to add to the list
// @param center: the point representing the center of the graph
// @param distance_coeff:   the coefficient that is applied when calculating distances from the
//                          center
// @return boolean: is the number added to the list and not offscren ?
fn add_number (numbers: &mut Vec<PNumber>, value: f64, center: &Point, distance_coeff: f64) -> bool {
    let x: f64 = calculate_xvalue(value);
    let y: f64 = calculate_yvalue(value);
    
    let dx = center.x + ((distance_coeff * x) as i32);
    let dy = center.y - ((distance_coeff * y) as i32);
    
    if dx > center.x * 2 || dy > center.y * 2 || dx < 0 || dy < 0
    {
        return false;
    }
    numbers.push(PNumber::create(x,y,value as u32, dx, dy));
    true
}

// print_number function: prints a PNumber instance on the graph. If we only want to print primes, 
// check if the PNumber instance is a prime number representation
// @param canvas: the surface on which we print the new number
// @param pt: the number to print
// @param print_only_prime: if true, we print the number only if it is a prime number
// @param cyan: cyan Color
// @param gold: gold Color
// @param distance_coeff:   the coefficient that is applied when calculating distances from the
//                          center
// @param center: the point representing the center of the graph
fn print_number(canvas: &mut WindowCanvas, pt: &mut PNumber, print_only_prime: bool,
               cyan: Color, gold: Color,
               distance_coeff: f64,
               center: &Point)
{

    if !pt.update(&center, distance_coeff)
    {
        return;
    }
        
    
    if !print_only_prime
    {
        if pt.prime
        {
            canvas.set_draw_color(cyan);
        }
        else
        {
            canvas.set_draw_color(gold);
        } 
        canvas.fill_rect(Rect::new(pt.dx, pt.dy, 2, 2));
    }
    else
    {
        if pt.prime {
            canvas.set_draw_color(cyan);
            canvas.fill_rect(Rect::new(pt.dx, pt.dy, 2, 2));
            }
    }
}


// exponential_extend_distance_coeff function: updates the distance coefficient
// the more bigger the distance coefficient is, the bigger it will be updated
// @param distance_coeff: the coefficient to update
// @return float 64 Bits: the updated coefficient
fn exponential_extend_distance_coeff(distance_coeff: f64) -> f64 {
    let mut tmp: f64 = 1.0;

    while tmp > distance_coeff {
        tmp /= 10.0;
    }
    distance_coeff + tmp
}
// logarithmic_dimin_distance_coeff function: updates the distance coefficient
// the more smaller the distance coefficient is, the smaller it will be updated
// @param distance_coeff: the coefficient to update
// @return float 64 Bits: the updated coefficient
fn logarithmic_dimin_distance_coeff(distance_coeff: f64) -> f64 {
    let mut tmp: f64 = 1000.0;

    while tmp > distance_coeff {
        tmp /= 10.0;
    }

    distance_coeff - tmp/ 10.0
}

pub fn main() {

    //sdl initialization
    let sdl_context = sdl2::init().unwrap();

    //video driver and window initialization
    let video_subsystem = sdl_context.video().unwrap();
    
    let width: u32 = 1800;
    let height: u32 = 1000;

    let window = video_subsystem.window("test", width, height)
        .position_centered()
        .build()
        .unwrap();
    

    //canvas and event handling initialization
    let mut canvas = window.into_canvas().build().unwrap();
 
    let mut event_pump = sdl_context.event_pump().unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();

    let texture_creator = canvas.texture_creator();
    

    let font_path: &Path = Path::new("font.ttf");
    // Load a font
    let mut font = ttf_context.load_font(font_path, 32).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);


    let black_color = Color::RGB(0,0,0);
    let white_color = Color::RGB(255,255,255);
    let gold_color = Color::RGB(255,210,0);
    let cyan_color = Color::RGB(0,255,255);
    
    let ordonnee = Rect::new((width / 2) as i32, 0,1, height);
    let abscisse = Rect::new(0, (height/2) as i32,width, 1);
    let center = Point::new((width/2) as i32, (height/2) as i32);
    
    let mut numbers: Vec<PNumber> = Vec::new();

    let mut distance_coeff: f64 = 1.0;

    let mut new_number: i32 = 1;

    let mut print_only_prime: bool = false;
    let mut show_axes: bool = false;
    

    let text_size: Rect = Rect::new(0, 0, 300, 40);

    //This loop runs as long as we do not break it through an event
    'running: loop {
        // render a surface, and convert it to a texture bound to the canvas
        let text: String = format!("Numbers generated: {}", new_number);

        let surface = font
            .render(&text)
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string()).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();
        
        if add_number(&mut numbers, new_number as f64, &center, distance_coeff)
        {
                new_number += 1;
        }

        canvas.set_draw_color(black_color);
        canvas.clear();         
        for mut pt in &mut numbers
        {
            print_number(&mut canvas, &mut pt, print_only_prime, 
                        cyan_color, gold_color,
                        distance_coeff,
                        &center);
        }

        canvas.set_draw_color(white_color);
        if show_axes
        {
            canvas.fill_rect(ordonnee);
            canvas.fill_rect(abscisse);
        }
        

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::N), .. } => {
                    print_only_prime = !print_only_prime;
                },
                Event::KeyDown{keycode: Some(Keycode::Down), .. } => {
                    distance_coeff = logarithmic_dimin_distance_coeff(distance_coeff);
                }
                Event::KeyDown{keycode: Some(Keycode::Up), .. } => {
                    distance_coeff = exponential_extend_distance_coeff(distance_coeff);
                }
                Event::KeyDown{keycode: Some(Keycode::A), .. } => {
                    show_axes = !show_axes;
                }
                _ => {}
           }
        }
        canvas.copy(&texture, None, Some(text_size));
        render(&mut canvas);

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120))
    }
} 
