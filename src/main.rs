use sfml::{graphics::*, window::*, system::*};
use std::ops::DerefMut;

enum Wall{
    North(f32),
    East(f32),
    South(f32),
    West(f32),
}

enum Collision{
    TwoBalls(usize,usize),
    CueBall(usize)
}

impl PartialEq for Collision{
    fn eq(&self,other:&Self) -> bool{
        match self{
            Collision::TwoBalls(i,j) => {
                match other{
                    Collision::TwoBalls(k,l)=> return i==k && j==l,
                    _ => return false,
                }
            }
            Collision::CueBall(i)=>{
                match other{
                    Collision::CueBall(k)=> return k==i,
                    _ => return false,
                }
            }
        }
    }
}

struct Options{
    border:f32,
    cursor_size:f32,
    start_x:f32,
    start_y:f32,
    num_dots:i32,
    num_balls:i8,
    force_multiplier:f32
}

struct State<'a>{
    field:RectangleShape<'a>,
    cue_ball:Ball<'a>,
    other_balls:Vec<Ball<'a>>,
    cursor:CircleShape<'a>,
    cursor_dots:Vec<CircleShape<'a>>,
    cursor_dot_sizes:Vec<f32>,
    recent_collisions:Vec<Collision>,
    options:Options
}

#[derive(Debug)]
struct Ball<'a>{
    shape:CircleShape<'a>,
    vx:f32,
    vy:f32,
}

impl Ball<'_>{
    const FRICTION:f32 = 100.0;
    const BALL_SIZE:f32 = 15.0;
    const BALL_RES:usize = 32;
    const PHYSICS_STEP:f32 = 1./60.;
    fn process(&mut self){
        self.shape.move_((self.vx*Ball::PHYSICS_STEP,self.vy*Ball::PHYSICS_STEP));

        let old_mag = (self.vx.abs()*self.vx.abs() + self.vy.abs()*self.vy.abs()).sqrt();
        let new_mag = old_mag - Ball::FRICTION*Ball::PHYSICS_STEP;
        if new_mag<0.0{
            self.vx = 0.;
            self.vy = 0.;
        }else{
            self.vy = new_mag * (self.vy/old_mag);
            self.vx = new_mag * (self.vx/old_mag);
        }

    }

    fn collision(& self,b2:& Self) -> ((f32,f32),(f32,f32)){
        let b1pos = self.shape.position();
        let b2pos = b2.shape.position();
        let dpx = b1pos.x-b2pos.x;
        let dpy = b1pos.y-b2pos.y;
        let pmag = (dpx*dpx+dpy*dpy).sqrt();

        let dirx = dpx/pmag;
        let diry = dpy/pmag;


        let dvx = self.vx-b2.vx;
        let dvy = self.vy-b2.vy;
        let transferx = dirx.abs()*dvx;
        let transfery = diry.abs()*dvy;
        
        
        let mut offsetx = 0.;
        let mut offsety = 0.;
        let desiredx = dirx*(Ball::BALL_SIZE+1.);
        let desiredy = diry*(Ball::BALL_SIZE+1.);

        if true {
            offsetx = dpx/2.-desiredx;
            offsety = dpy/2.-desiredy;
        }
        


        return ((transferx,transfery),(offsetx,offsety));
    }
    fn wall_collision(&mut self,wall:Wall){
        match wall{
            Wall::East(num)  => {
                self.vx = -self.vx.abs();
                while self.shape.position().x>num{
                    self.shape.move_((-Ball::FRICTION*Ball::PHYSICS_STEP,0.));
                }
            },
            Wall::North(num) => {
                self.vy = self.vy.abs();
                while self.shape.position().y<num{
                    self.shape.move_((0.,Ball::FRICTION*Ball::PHYSICS_STEP));
                }
            },
            Wall::South(num) => {
                self.vy = -self.vy.abs();
                while self.shape.position().y>num{
                    self.shape.move_((0.,-Ball::FRICTION*Ball::PHYSICS_STEP));
                }
            },
            Wall::West(num)  => {
                self.vx = self.vx.abs();
                while self.shape.position().x<num{
                    self.shape.move_((self.vx*Ball::PHYSICS_STEP,0.));
                }
            }
        }
    }

    fn check_hit_wall(ball:&mut Ball,walls:& RectangleShape){

        let ball_pos = ball.shape.position();
        let field_bounds = walls.global_bounds();
        let low_ball = ball_pos.y;
        let high_ball = ball_pos.y;
        let left_ball = ball_pos.x;
        let right_ball = ball_pos.x;
        let bottom_wall:f32 = field_bounds.top+field_bounds.height-2.*Ball::BALL_SIZE;
        let top_wall:f32 = field_bounds.top;
        let left_wall:f32 = field_bounds.left;
        let right_wall:f32 = field_bounds.left + field_bounds.width-2.*Ball::BALL_SIZE;
        if low_ball>bottom_wall{
            ball.wall_collision(Wall::South(bottom_wall));
        }
        if high_ball<top_wall{
            ball.wall_collision(Wall::North(top_wall));
        }
        if left_ball<left_wall{
            ball.wall_collision(Wall::West(left_wall));
        }
        if right_ball>right_wall{
            ball.wall_collision(Wall::East(right_wall));
        }
    }

    fn check_hit_ball(ball1:&Ball,ball2:&Ball,col:Option<Collision>) -> Option<((f32,f32),(f32,f32))>{
        let b1_center = ball1.shape.position();
        let b2_center = ball2.shape.position();
    
        let dx = (b2_center.x-b1_center.x).abs();
        let dy = (b2_center.y-b1_center.y).abs();
        
        if (dx*dx+dy*dy).sqrt()<Ball::BALL_SIZE*2.{
            let ((a,b),(c,d)) = ball1.collision(ball2);
            match col{
                Some(_) => return Some(((0.,0.),(c,d))),
                None => return Some(((a,b),(c,d))),
            }
            
        } else {
            return None;
        }
    }
}

fn lerp(start:f32,stop:f32,ratio:f32)->f32{
    let range = stop-start;
    let terp = range*ratio;
    return start+terp;
}


fn calculate_physics(state:&mut State){
    // process movement
    // check wall collisions 
    // check ball collisions
        
    state.cue_ball.process();
    
    
    for i in 0..state.other_balls.len(){
        state.other_balls[i].process();
        
        let col_type = Collision::CueBall(i);
        let mut op = None;
        if state.recent_collisions.contains(&col_type){
            state.recent_collisions.retain(|x| x != &col_type);
            op = Some(col_type);
        }
        match Ball::check_hit_ball(&mut state.other_balls[i],&mut state.cue_ball,op){
            Some(((x,y),(dx,dy))) => {
                state.other_balls[i].vx -= x;
                state.other_balls[i].vy -= y;
                state.cue_ball.vx += x;
                state.cue_ball.vy += y;
                state.other_balls[i].shape.move_((-dx,-dy));
                state.cue_ball.shape.move_((dx,dy));
                state.recent_collisions.push(Collision::CueBall(i));
            },
            None => {}
        }

        for j in (i+1)..state.other_balls.len(){
            let col_type = Collision::TwoBalls(i,j);
            let mut op = None;
            if state.recent_collisions.contains(&col_type){
                state.recent_collisions.retain(|x| x != &col_type);
                op = Some(col_type);
            }
            match Ball::check_hit_ball(& state.other_balls[i],&state.other_balls[j],op){
                Some(((x,y),(dx,dy))) => {
                    state.other_balls[i].vx -= x;
                    state.other_balls[i].vy -= y;
                    state.other_balls[j].vx += x;
                    state.other_balls[j].vy += y;
                    state.other_balls[i].shape.move_((-dx,-dy));
                    state.other_balls[j].shape.move_((dx,dy));
                    state.recent_collisions.push(Collision::TwoBalls(i,j));
                },
                None => {}
            }
        }
        Ball::check_hit_wall(&mut state.other_balls[i],&state.field);
    }
    Ball::check_hit_wall(&mut state.cue_ball,&state.field);
}

fn update_cursor(window:&RenderWindow,state:&mut State){
    let mc = window.mouse_position();
    state.cursor.set_position((mc.x as f32-state.options.cursor_size, mc.y as f32-state.options.cursor_size));

    let dots = state.cursor_dots.len();
    let mut cb = state.cue_ball.shape.position();
    cb.x += Ball::BALL_SIZE;
    cb.y += Ball::BALL_SIZE;
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
    //println!("{:?}",state.cue_ball.shape.position());
    for dot in state.cursor_dots.iter(){
        window.draw(dot);
    }
    for ball in &state.other_balls{
        window.draw(&ball.shape);
    }
    window.draw(&state.cursor);
    window.display();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut w_binding = RenderWindow::new(
      (800,600),
      "Virtual Pool",
      Style::CLOSE,
      &Default::default()
    )?;
    let mut window = w_binding.deref_mut();
    window.set_mouse_cursor_visible(false);
    let options = Options { 
        border: 35.0, 
        cursor_size: 6.0, 
        start_x: 100.0, 
        start_y: 100.0,
        num_dots: 3,
        num_balls:10,
        force_multiplier:10.
    };
    let mut dots:Vec<CircleShape> = Vec::new();
    let mut dot_sizes:Vec<f32> = vec![];
    for i in 1..=options.num_dots{
        let size = lerp(1.,options.cursor_size,i as f32/(options.num_dots+1)as f32);
        let dot = CircleShape::new(size,Ball::BALL_RES);
        dots.push(dot);
        dot_sizes.push(size);
    }

    let mut balls = vec![];
    for i in 0..options.num_balls{
        let mut ball = Ball{shape:CircleShape::new(Ball::BALL_SIZE,Ball::BALL_RES),vx:0.,vy:0.};
        ball.shape.set_position(((((i+1) as i32*100).rem_euclid(600) as f32)+100., (((i+1) as i32/7) as f32)*100.+200.));
        ball.shape.set_fill_color(Color::RED);
        balls.push(ball)

    }

    let mut state = State{
        field:RectangleShape::from_rect(Rect{left:options.border,top:options.border,
            width:window.size().x as f32-2.0*options.border,
            height:window.size().y as f32-2.0*options.border}),
        cue_ball:Ball { shape: CircleShape::new(Ball::BALL_SIZE,Ball::BALL_RES), vx: 0.0, vy: 0.0},
        other_balls:balls,
        cursor:CircleShape::new(options.cursor_size,Ball::BALL_RES),
        cursor_dots:dots,
        cursor_dot_sizes:dot_sizes,
        options:options,
        recent_collisions:vec![],
    };
    state.field.set_fill_color(Color::rgb(0,100,0));
    state.cue_ball.shape.set_position((state.options.start_x,state.options.start_y));

    let mut c_binding = Clock::start()?;
    let clock = c_binding.deref_mut();

    loop{
        while let Some(ev) = window.poll_event(){
            match ev {
                Event::Closed => return Ok(()),
                Event::MouseButtonPressed { button:_, x:_, y:_ } => {
                    let mc = window.mouse_position();
                    let mut cb = state.cue_ball.shape.position();
                    cb.x += Ball::BALL_SIZE;
                    cb.y += Ball::BALL_SIZE;

                    state.cue_ball.vx += (cb.x-mc.x as f32)*state.options.force_multiplier;
                    state.cue_ball.vy += (cb.y-mc.y as f32)*state.options.force_multiplier;
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
}
