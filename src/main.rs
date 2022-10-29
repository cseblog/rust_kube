// Define the screen and speed of rotations around x, y, z axises
const SCREEN_H: usize = 80;
const SCREEN_W: usize = 160;
const HALF_KSIZE: f64 = 20.0;
const K1: usize = 40;
const INCREAMENT_SPEED: f64 = 0.6;
const ROTATE_X_SPEED: f64 = 0.04;
const ROTATE_Y_SPEED: f64 = 0.04;
const ROTATE_Z_SPEED: f64 = 0.04;
/// float calculateX(int i, int j, int k) {
/// return j * sin(A) * sin(B) * cos(C) - k * cos(A) * sin(B) * cos(C) +
/// j * cos(A) * sin(C) + k * sin(A) * sin(C) + i * cos(B) * cos(C);
/// }

/// float calculateY(int i, int j, int k) {
/// return j * cos(A) * cos(C) + k * sin(A) * cos(C) -
/// j * sin(A) * sin(B) * sin(C) + k * cos(A) * sin(B) * sin(C) -
/// i * cos(B) * sin(C);
/// }

/// float calculateZ(int i, int j, int k) {
/// return k * cos(A) * cos(B) - j * sin(A) * cos(B) + i * sin(B);
/// }

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

//This require update 2d array from a function in RUST, very tricky
fn update<V: AsMut<[f64]>, K: AsMut<[char]>>(
    ch: char,
    zbuffer: &mut [V],
    output: &mut [K],
    (x, y, ooz, idx): (usize, usize, f64, usize),
) {
    // if x < SCREEN_H as usize && y < SCREEN_W as usize {
    // output[x].as_mut()[y] = ch;
    // }

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
    let xp = (30.0 + HALF_KSIZE + K1 as f64 * ooz * x) as usize;
    let yp = (30.0 + HALF_KSIZE + K1 as f64 * ooz * y) as usize;
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
        let mut cx = -HALF_KSIZE;
        while cx < HALF_KSIZE {
            let mut cy = -HALF_KSIZE;
            while cy < HALF_KSIZE {
                // Start calculate 6 surfaces of Kube
                let (x, y, ooz, idx) = calculate_for_surface(cx, cy, -HALF_KSIZE, a, b, c); // a, b, c are angles of rotations
                                                                                            // Update the the screen with character use about output
                update('.', &mut zbuffer, &mut output, (x, y, ooz, idx));

                let (x, y, ooz, idx) = calculate_for_surface(cx, cy, HALF_KSIZE, a, b, c); // a, b, c are angles of rotations
                                                                                           // Update the the screen with character use about output
                update('#', &mut zbuffer, &mut output, (x, y, ooz, idx));

                let (x, y, ooz, idx) = calculate_for_surface(HALF_KSIZE, cx, cy, a, b, c); // a, b, c are angles of rotations
                                                                                           // Update the the screen with character use about output
                update('$', &mut zbuffer, &mut output, (x, y, ooz, idx));

                let (x, y, ooz, idx) = calculate_for_surface(-HALF_KSIZE, cx, cy, a, b, c); // a, b, c are angles of rotations
                                                                                            // Update the the screen with character use about output
                update('~', &mut zbuffer, &mut output, (x, y, ooz, idx));

                let (x, y, ooz, idx) = calculate_for_surface(cx, HALF_KSIZE, cy, a, b, c); // a, b, c are angles of rotations
                                                                                           // Update the the screen with character use about output
                update(';', &mut zbuffer, &mut output, (x, y, ooz, idx));

                let (x, y, ooz, idx) = calculate_for_surface(cx, -HALF_KSIZE, cy, a, b, c); // a, b, c are angles of rotations
                                                                                            // Update the the screen with character use about output
                update('+', &mut zbuffer, &mut output, (x, y, ooz, idx));

                cy += INCREAMENT_SPEED;
            }
            cx += INCREAMENT_SPEED;
        }

        //SHow to SCREEN
        print!("\x1b[H"); //clear screen
        for i in 0..SCREEN_H as usize {
            for j in 0..SCREEN_W as usize {
                print!("{}", output[i][j]);
            }
            print!("\n");
        }

        a += ROTATE_X_SPEED;
        b += ROTATE_Y_SPEED;
        c += ROTATE_Z_SPEED;

        std::thread::sleep(std::time::Duration::new(0, 30000000));
    }
}
