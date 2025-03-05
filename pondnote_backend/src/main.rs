use std::{thread, time::Duration};

fn main() {
    let mut angle = 0.0f64;
    loop {
        print!("\x1B[2J\x1B[1;1H"); // 清屏
        let heart = generate_rotating_heart(angle, 20.0);
        println!("{}", heart);
        
        angle += 0.1;
        if angle >= 360.0 {
            angle = 0.0;
        }
        
        thread::sleep(Duration::from_millis(50));
    }
}

fn generate_rotating_heart(angle: f64, size: f64) -> String {
    let mut result = String::new();
    let angle_rad = angle.to_radians();
    
    for y in (-15..15).rev() {
        for x in -30..30 {
            let x_f = x as f64 / size;
            let y_f = y as f64 / size;
            
            // 旋转变换
            let x_rot = x_f * angle_rad.cos() - y_f * angle_rad.sin();
            let y_rot = x_f * angle_rad.sin() + y_f * angle_rad.cos();
            
            // 爱心方程
            let heart_eq = (x_rot * x_rot + y_rot * y_rot - 1.0).powf(3.0) 
                - x_rot * x_rot * y_rot * y_rot * y_rot;
            
            if heart_eq <= 0.0 {
                result.push('❤');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }
    result
}
