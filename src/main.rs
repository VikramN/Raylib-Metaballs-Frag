use raylib::prelude::*;
use grid::Grid;
use metaballs::Metaball;

mod grid;
mod metaballs;

fn main() {

    let screen = (800, 600);
    let bounds = Vector2 { x : screen.0 as f32, y : screen.1 as f32 };

    let (mut rl, thread) = raylib::init()
        .log_level(TraceLogLevel::LOG_NONE)
        .size(screen.0, screen.1)
        .title("Hello, Metaballs")
        .build();

    let mut blobs = vec![
        Metaball::new(bounds),
        Metaball::new(bounds),
        Metaball::new(bounds),
        Metaball::new(bounds),
        Metaball::new(bounds)
    ];

    let grid = Grid { color : Color::alpha(&Color::WHITE, 0.5), size : 50 };

    let shader_code = r#"
        #version 330

        // Input vertex attributes (from vertex shader)
        in vec2 fragTexCoord;
        in vec4 fragColor;

        // Input uniform values
        uniform sampler2D texture0;
        uniform vec4 colDiffuse;
        uniform vec2 screen;
        uniform vec3 balls[50];
        uniform int ballCount;

        // Output fragment color
        out vec4 finalColor;

        // NOTE: Add here your custom variables

        void main()
        {
            float d = 0.0;

            for (int i = 0; i < ballCount; i++) {
                vec3 ball = balls[i];
                vec2 p = ball.xy;
                p.y = screen.y -p.y;
                d += ball.z / distance(gl_FragCoord.xy, p) * 50.0;
            }

            float u = step(100.0, d) * 200.0 / 255.0;
            finalColor = vec4(u, u, u ,1.0);
        }
    "#;

    let mut shader = rl.load_shader_from_memory(&thread, None, Some(shader_code.to_string().as_str()));
    
    shader.set_shader_value(
        shader.get_shader_location("screen"),
        [bounds.x, bounds.y]);

    while !rl.window_should_close() {

        let elapsed = rl.get_frame_time();

        for b in blobs.iter_mut() {
            b.update(elapsed);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if blobs.len() < 50 && d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            blobs.push( *Metaball::new(bounds).move_to(d.get_mouse_position()) );
        }

        set_shader_values(&blobs, &mut shader);

        {
            let mut y = d.begin_shader_mode(&shader);
            y.draw_rectangle(0, 0, screen.0, screen.1, Color::alpha(&Color::WHITE, 1.0));
        }

        grid.draw(screen.0, screen.1, &mut d);
        d.draw_text("LEFT-CLICK to ADD", 10, screen.1 - 50, 22, Color::RED);
        d.draw_fps(screen.0 - 100, screen.1 - 50);

    }
}

fn set_shader_values(blobs : &[Metaball], shader : &mut Shader) {
    let d : Vec<Vector3> = blobs.iter().map(|b| { b.to_vec3()}).collect();

    shader.set_shader_value_v(
        shader.get_shader_location("balls"),
        &d);

    shader.set_shader_value(
        shader.get_shader_location("ballCount"),
        d.len() as i32);
}
