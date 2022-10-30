// Define the screen and speed of rotations around x, y, z axises
const SCREEN_H: usize = 80;
const SCREEN_W: usize = 160;
const HALF_CUBE_SIZE: f64 = 20.0;
const K1: usize = 40;
const INCREAMENT_SPEED: f64 = 0.6;
const ROTATE_AROUND_X_AXIS_SPEED: f64 = 0.04;
const ROTATE_AROUND_Y_AXIS_SPEED: f64 = 0.04;
const ROTATE_AROUND_Z_AXIS_SPEED: f64 = 0.04;

fn rotate_x(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64) -> f64 {
    let (sina, cosa) = f64::sin_cos(a);
    let (sinb, cosb) = f64::sin_cos(b);
    let (sinc, cosc) = f64::sin_cos(c);
    return y * sina * sinb * cosc - z * cosa * sinb * cosc
        + y * cosa * sinc
        + z * sina * sinc
        + x * cosb * cosc;
}

fn rotate_y(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64) -> f64 {
    let (sina, cosa) = f64::sin_cos(a);
    let (sinb, cosb) = f64::sin_cos(b);
    let (sinc, cosc) = f64::sin_cos(c);
    return y * cosa * cosc + z * sina * cosc - y * sina * sinb * sinc + z * cosa * sinb * sinc
        - x * cosb * sinc;
}

fn rotate_z(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64) -> f64 {
    let (sina, cosa) = f64::sin_cos(a);
    let (sinb, cosb) = f64::sin_cos(b);
    let (sinc, cosc) = f64::sin_cos(c);
    return z * cosa * cosb - y * sina * cosb + x * sinb;
}

//This require set_character_at_coordinate 2d array from a function in RUST, very tricky
fn set_character_at_coordinate<V: AsMut<[f64]>, K: AsMut<[char]>>(
    ch: char,
    zbuffer: &mut [V],
    output: &mut [K],
    (x, y, ooz, idx): (usize, usize, f64, usize),
) {
    //Add luminance effect to the kube
    if idx > 0
        && idx < (SCREEN_H * SCREEN_W) as usize
        && x < SCREEN_H as usize
        && y < SCREEN_W as usize
    {
        if ooz > zbuffer[x].as_mut()[y] {
            zbuffer[x].as_mut()[y] = ooz;
            output[x].as_mut()[y] = ch;
        }
    }
}

fn calculate_for_surface(
    cx: f64,
    cy: f64,
    cz: f64,
    a: f64,
    b: f64,
    c: f64,
) -> (usize, usize, f64, usize) {
    let distance_from_eyes = 100.0;
    let x = rotate_x(cx, cy, cz, a, b, c);
    let y = rotate_y(cx, cy, cz, a, b, c);
    let z = rotate_z(cx, cy, cz, a, b, c) + distance_from_eyes;

    let ooz = 1.0 / z;
    let xp = (30.0 + HALF_CUBE_SIZE + K1 as f64 * ooz * x) as usize;
    let yp = (30.0 + HALF_CUBE_SIZE + K1 as f64 * ooz * y) as usize;
    let idx = xp + yp * SCREEN_W;
    return (xp, yp, ooz, idx);
}

fn main() {
    print!("\x1b[2J");
    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;
   loop {
        let mut output = [[' '; SCREEN_W]; SCREEN_H]; //SCREEN
        let mut zbuffer = [[0.0; SCREEN_W]; SCREEN_H]; //SCREEN
        let mut cx = -HALF_CUBE_SIZE;
        while cx < HALF_CUBE_SIZE {
            let mut cy = -HALF_CUBE_SIZE;
            while cy < HALF_CUBE_SIZE {
                // Start calculate 6 surfaces of Kube
                let (x, y, ooz, idx) = calculate_for_surface(cx, cy, -HALF_CUBE_SIZE, a, b, c); // a, b, c are angles of rotations
                set_character_at_coordinate('.', &mut zbuffer, &mut output, (x, y, ooz, idx));

                let (x, y, ooz, idx) = calculate_for_surface(cx, cy, HALF_CUBE_SIZE, a, b, c); 
                set_character_at_coordinate('#', &mut zbuffer, &mut output, (x, y, ooz, idx));
                
                let (x, y, ooz, idx) = calculate_for_surface(HALF_CUBE_SIZE, cx, cy, a, b, c);
                set_character_at_coordinate('$', &mut zbuffer, &mut output, (x, y, ooz, idx));
                
                let (x, y, ooz, idx) = calculate_for_surface(-HALF_CUBE_SIZE, cx, cy, a, b, c);
                set_character_at_coordinate('~', &mut zbuffer, &mut output, (x, y, ooz, idx));
                
                let (x, y, ooz, idx) = calculate_for_surface(cx, HALF_CUBE_SIZE, cy, a, b, c);
                set_character_at_coordinate(';', &mut zbuffer, &mut output, (x, y, ooz, idx));

                let (x, y, ooz, idx) = calculate_for_surface(cx, -HALF_CUBE_SIZE, cy, a, b, c);
                set_character_at_coordinate('+', &mut zbuffer, &mut output, (x, y, ooz, idx));

                cy += INCREAMENT_SPEED;
            }
            cx += INCREAMENT_SPEED;
        }

        print!("\x1b[H"); //clear screen
        for i in 0..SCREEN_H as usize {
            for j in 0..SCREEN_W as usize {
                print!("{}", output[i][j]);
            }
            print!("\n");
        }

        a += ROTATE_AROUND_X_AXIS_SPEED;
        b += ROTATE_AROUND_Y_AXIS_SPEED;
        c += ROTATE_AROUND_Z_AXIS_SPEED;

        std::thread::sleep(std::time::Duration::new(0, 30000000));
    }
}
