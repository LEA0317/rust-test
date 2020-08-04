union Conv32 {
      i: i32,
      f: f32,
}

fn sqrtf(x: f32) -> f32 {
   let half_x: f32 = x * 0.5;
   let half3: f32  = 1.5;

   let mut conv = Conv32{ f: x };
   unsafe {
     conv.i = 0x5f3759df - (conv.i >> 1);
     conv.f *= half3 - (half_x * conv.f * conv.f);
     conv.f *= half3 - (half_x * conv.f * conv.f);
     conv.f *= half3 - (half_x * conv.f * conv.f);
     conv.f * x
   }
}

fn sqrtd(x: f64) -> f64 {
   let half_x: f64 = x * 0.5;
   let half3: f64  = 1.5;

   let mut conv = Conv32{ f: x as f32 };
   let mut rsqrt: f64;
   unsafe {
     conv.i = 0x5f3759df - (conv.i >> 1);
     rsqrt = conv.f as f64;
     rsqrt *= half3 - (half_x * rsqrt * rsqrt);
     rsqrt *= half3 - (half_x * rsqrt * rsqrt);
     rsqrt *= half3 - (half_x * rsqrt * rsqrt);
     rsqrt *= half3 - (half_x * rsqrt * rsqrt);
     rsqrt *= half3 - (half_x * rsqrt * rsqrt);
     rsqrt * x
   }
}

fn main() {
    println!("{}", sqrtf(3.0));
    println!("{}", sqrtd(3.0));
}
