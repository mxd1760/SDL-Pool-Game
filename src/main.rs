use sfml::{graphics::*, window::*, system::*};

enum Wall{
    North,
    East,
    South,
    West,
}

struct Options{
    border:f32,
    ball_size:f32,
    cursor_size:f32,
    ball_res:usize,
    start_x:f32,
    start_y:f32,
    num_dots:i32,
}

struct State<'a>{
    field:RectangleShape<'a>,
    cue_ball:Ball<'a>,
    cursor:CircleShape<'a>,
    cursor_dots:Vec<CircleShape<'a>>,
    cursor_dot_sizes:Vec<f32>,
    options:Options
}

#[derive(Debug)]
struct Ball<'a>{
    shape:CircleShape<'a>,
    vx:f32,
    vy:f32,
}

impl Ball<'_>{
    const FRICTION:f32 = 30.0;
    fn process(&mut self){
        self.shape.move_((self.vx*(1./60.),self.vy*(1./60.)));

        let mag_x = self.vx.abs() - Ball::FRICTION*(1./60.);
        if mag_x<0.0{
            self.vx=0.0;
        }else{
            if self.vx>0.{
                self.vx = mag_x;
            }else{
                self.vx = -mag_x;
            }
        }
        let mag_y = self.vy.abs() - Ball::FRICTION*(1./60.);
        if mag_y<0.0 {
            self.vy=0.0
        }else{
            if self.vy>0.{
                self.vy=mag_y;
            }else{
                self.vy=-mag_y;
            }
        }
    }
    // fn collision(&mut self,other:&mut Self){

    // }
    fn wall_collision(&mut self,wall:Wall){
        match wall{
            Wall::East  => {self.vx = -self.vx.abs()},
            Wall::North => {self.vy = self.vy.abs()},
            Wall::South => {self.vy = -self.vy.abs()},
            Wall::West  => {self.vx = self.vx.abs()}
        }
    }
}

fn lerp(start:f32,stop:f32,ratio:f32)->f32{
    let range = stop-start;
    let terp = range*ratio;
    return start+terp;
}

fn check_ball_hit_wall(ball:&mut Ball,ball_radius:f32,walls:& RectangleShape){

    let ball_pos = ball.shape.position();
    let field_bounds = walls.global_bounds();
    let low_ball = ball_pos.y+2.*ball_radius;
    let high_ball = ball_pos.y;
    let left_ball = ball_pos.x;
    let right_ball = ball_pos.x+2.*ball_radius;
    let bottom_wall = field_bounds.top+field_bounds.height;
    let top_wall = field_bounds.top;
    let left_wall = field_bounds.left;
    let right_wall = field_bounds.left + field_bounds.width;
    if low_ball>bottom_wall{
        ball.wall_collision(Wall::South);
    }
    if high_ball<top_wall{
        ball.wall_collision(Wall::North);
    }
    if left_ball<left_wall{
        ball.wall_collision(Wall::West);
    }
    if right_ball>right_wall{
        ball.wall_collision(Wall::East);
    }
}

fn calculate_physics(state:&mut State){ // TODO make multiple balls
    // process movement
    state.cue_ball.process();

    // check wall collisions 
    check_ball_hit_wall(&mut state.cue_ball,state.options.ball_size,&state.field);


    // check ball collisions
}

fn update_cursor(window:&RenderWindow,state:&mut State){
    let mut mc = window.mouse_position();
    state.cursor.set_position((mc.x as f32-state.options.cursor_size, mc.y as f32-state.options.cursor_size));

    let dots = state.cursor_dots.len();
    let mut cb = state.cue_ball.shape.position();
    cb.x += state.options.ball_size;
    cb.y += state.options.ball_size;
    for i in 0..dots{
        let factor = (i+1)as f32/(dots+1) as f32;
        let dot = &mut state.cursor_dots[i];
        let x = lerp(cb.x,mc.x as f32,factor)-state.cursor_dot_sizes[i];
        let y = lerp(cb.y,mc.y as f32,factor)-state.cursor_dot_sizes[i];
        dot.set_position((x,y));
    }
}


fn draw(window:&mut RenderWindow,state:&State){
    window.clear(Color::rgb(100,50,20));
    window.draw(&state.field);
    window.draw(&state.cue_ball.shape);
    for dot in state.cursor_dots.iter(){
        window.draw(dot);
    }
    window.draw(&state.cursor);
    window.display();
}

fn main() {
    let mut window = RenderWindow::new(
        (800,600),
        "Mouse events",
        Style::CLOSE,
        &Default::default()
    );
    window.set_mouse_cursor_visible(false);
    let options = Options { 
        border: 35.0, 
        ball_size:15.0, 
        cursor_size: 6.0, 
        ball_res: 32, 
        start_x: 100.0, 
        start_y: 100.0,
        num_dots: 3
    };
    let mut dots:Vec<CircleShape> = Vec::new();
    let mut dot_sizes:Vec<f32> = vec![];
    for i in 1..=options.num_dots{
        let size = lerp(1.,options.cursor_size,i as f32/(options.num_dots+1)as f32);
        let dot = CircleShape::new(size,options.ball_res);
        dots.push(dot);
        dot_sizes.push(size);
    }
    let mut state = State{
        field:RectangleShape::from_rect(Rect{left:options.border,top:options.border,
            width:window.size().x as f32-2.0*options.border,
            height:window.size().y as f32-2.0*options.border}),
        cue_ball:Ball { shape: CircleShape::new(options.ball_size,options.ball_res), vx: 0.0, vy: 0.0},
        cursor:CircleShape::new(options.cursor_size,options.ball_res),
        cursor_dots:dots,
        cursor_dot_sizes:dot_sizes,
        options:options,
    };
    state.field.set_fill_color(Color::rgb(0,100,0));
    state.cue_ball.shape.set_position((state.options.start_x,state.options.start_y));
    //window.set_vertical_sync_enabled(true);

    //let font = Font::from_file("C:\\windows\\Fonts\\BIZ-UDGothicB.ttc").unwrap();

    let mut clock = Clock::start();

    loop{
        while let Some(ev) = window.poll_event(){
            match ev {
                Event::Closed => return,
                Event::MouseButtonPressed { button:_, x:_, y:_ } => {
                    let mc = window.mouse_position();
                    let mut cb = state.cue_ball.shape.position();
                    cb.x += state.options.ball_size;
                    cb.y += state.options.ball_size;

                    state.cue_ball.vx += cb.x-mc.x as f32;
                    state.cue_ball.vy += cb.y-mc.y as f32;
                    //println!("cue_ball ({},{})",state.cue_ball.vx,state.cue_ball.vy);
                }
                _=>{}
            }
        }
        update_cursor(&window, &mut state);

        if clock.elapsed_time().as_milliseconds() as f32/1000. > (1./60.){
            calculate_physics(&mut state);
            clock.restart();
        }
        
        draw(&mut window,&state);
    }
    // let mut circle = CircleShape::new(4.,30);
    // let mut texts:Vec<Text> = Vec::new();
    // let mut mp_text = Text::new("",&font, 14);
    // let mut cursor_visible = false;
    // let mut grabbed = false;
    // macro_rules! push_text{
    //     ($x:expr, $y:expr, $fmt:expr, $($arg:tt)*)=>{
    //         let mut text = Text::new(&format!($fmt, $($arg)*), &font, 14);
    //         text.set_position(($x as f32, $y as f32));
    //         texts.push(text);
    //     }
    // }

    // loop {
    //     while let Some (ev) = window.poll_event(){
    //         match ev {
    //             Event::Closed => return,
    //             Event::MouseWheelScrolled {wheel, delta, x, y } =>{
    //                 push_text!(x,y, "Scroll: {:?}, {}, {}, {}",wheel, delta, x, y);
    //             }
    //             Event::MouseButtonPressed { button, x, y} => {
    //                 push_text!(x,y,"Press: {:?}, {}, {}",button, x, y);
    //             }
    //             Event::MouseButtonReleased { button, x, y } => {
    //                 push_text!(x,y,"Release: {:?}, {}, {}",button,x,y);
    //             }
    //             Event::KeyPressed {code, ..}=>{
    //                 if code == Key::W{
    //                     window.set_mouse_position(Vector2i::new(400,300));
    //                 }else if code == Key::D{
    //                     let dm = VideoMode::desktop_mode();
    //                     let center = Vector2i::new(dm.width as i32/2, dm.height as i32/2);
    //                     mouse::set_desktop_position(center);
    //                 }else if code == Key::V{
    //                     cursor_visible = !cursor_visible;
    //                     window.set_mouse_cursor_visible(cursor_visible);
    //                 }else if code == Key::G{
    //                     grabbed = !grabbed;
    //                     window.set_mouse_cursor_grabbed(grabbed);
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    //     let mp = window.mouse_position();
    //     let dmp = mouse::desktop_position();
    //     let cur_vis_msg = if cursor_visible {
    //         "visible"
    //     } else {
    //         "invisible"
    //     };
    //     let grab_msg = if grabbed { "grabbed" } else { "not grabbed" };
    //     mp_text.set_string(&format!(
    //         "x: {}, y: {} (Window)\n\
    //          x: {}, y: {} (Desktop)\n\
    //          [{cur_vis_msg}] [{grab_msg}] ('v'/'g') to toggle\n\
    //          'w' to center mouse on window\n\
    //          'd' to center mouse on desktop",
    //          mp.x, mp.y, dmp.x, dmp.y
    //     ));

    //     circle.set_position((mp.x as f32, mp.y as f32));

    //     window.clear(Color::BLACK);

    //     for i in (0..texts.len()).rev(){
    //         for j in (0..i).rev() {
    //             if let Some(intersect) = texts[i]
    //                 .global_bounds()
    //                 .intersection(&texts[j].global_bounds())
    //             {
    //                 texts[j].move_((0., -intersect.height));
    //             }
    //         }
    //     }
    //     texts.retain(|txt| txt.fill_color().a >0);
    //     for txt in &mut texts {
    //         let mut color = txt.fill_color();
    //         color.a -= 1;
    //         txt.set_fill_color(color);
    //         window.draw(txt);
    //     }
    //     if !cursor_visible{
    //         window.draw(&circle);
    //     }
    //     window.draw(&mp_text);
    //     window.display();
    // }
}
