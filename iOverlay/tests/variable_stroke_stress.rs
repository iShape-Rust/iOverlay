use i_overlay::mesh::variable_stroke::offset::VariableStrokeOffset;
use i_overlay::mesh::variable_stroke::{StrokeVertex, VariableStrokeStyle};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::{Duration, Instant};

#[test]
fn randomized_variable_stroke_does_not_panic() {
    for iteration in 0..512 {
        let seed = next_stress_seed(iteration as u64);
        run_variable_stroke_stress_case(seed, iteration);
    }
}

#[test]
fn disconnected_drawable_sections_do_not_build_join_between_centers() {
    let seed = 11_958_792_495_002_733_140;
    run_variable_stroke_stress_case(seed, 163);
}

#[test]
#[ignore = "long randomized variable-stroke stress test"]
fn randomized_variable_stroke_stress() {
    let seconds = env_u64("VARIABLE_STROKE_STRESS_SECONDS", 600);
    let mut seed = env_u64("VARIABLE_STROKE_STRESS_SEED", 0xa076_1d64_78bd_642f);
    let deadline = Instant::now() + Duration::from_secs(seconds);
    let mut iteration = 0usize;

    while Instant::now() < deadline {
        run_variable_stroke_stress_case(seed, iteration);
        seed = next_stress_seed(seed);
        iteration += 1;
    }

    eprintln!("variable-stroke stress completed: iterations={iteration} seconds={seconds}");
}

fn run_variable_stroke_stress_case(seed: u64, iteration: usize) {
    let mut rng = StressRng::new(seed);
    let path = random_variable_stroke_path(&mut rng);
    let round_angle = rng.range_f32(0.01, 0.8);
    let style = VariableStrokeStyle::new().round_angle(round_angle);

    let result = catch_unwind(AssertUnwindSafe(|| {
        let shapes = path.variable_stroke(style);
        assert_valid_shapes(&shapes, seed);

        if seed & 1 == 0 {
            let shapes = path.variable_stroke_as::<i64>(style);
            assert_valid_shapes(&shapes, seed);
        }

        let mut reversed = path.clone();
        reversed.reverse();
        let shapes = reversed.variable_stroke(style);
        assert_valid_shapes(&shapes, seed);
    }));

    if let Err(payload) = result {
        panic!(
            "variable-stroke stress failed: iteration={iteration} seed={seed} path={path:?} panic={}",
            panic_message(payload)
        );
    }
}

fn random_variable_stroke_path(rng: &mut StressRng) -> Vec<StrokeVertex<[f32; 2]>> {
    let count = rng.range_usize(3, 16);
    let mut path = Vec::with_capacity(count);
    let mut point = [rng.range_f32(-200.0, 200.0), rng.range_f32(-200.0, 200.0)];
    let mut direction = rng.range_f32(0.0, 2.0 * core::f32::consts::PI);

    for index in 0..count {
        let width = match rng.next_u32() % 12 {
            0 => 0.0,
            1 => rng.range_f32(0.0001, 0.01),
            2 => rng.range_f32(200.0, 600.0),
            _ => rng.range_f32(0.01, 200.0),
        };
        path.push(StrokeVertex::new(point, width));

        if index + 1 == count || rng.next_u32() % 20 == 0 {
            continue;
        }

        let turn = match rng.next_u32() % 6 {
            0 => core::f32::consts::PI + rng.range_f32(-0.02, 0.02),
            1 => core::f32::consts::FRAC_PI_2 + rng.range_f32(-0.02, 0.02),
            2 => -core::f32::consts::FRAC_PI_2 + rng.range_f32(-0.02, 0.02),
            _ => rng.range_f32(-core::f32::consts::PI, core::f32::consts::PI),
        };
        direction += turn;

        let length = match rng.next_u32() % 8 {
            0 => rng.range_f32(0.0001, 0.01),
            _ => rng.range_f32(0.01, 160.0),
        };
        point[0] += length * direction.cos();
        point[1] += length * direction.sin();
    }

    path
}

fn assert_valid_shapes(shapes: &[Vec<Vec<[f32; 2]>>], seed: u64) {
    for contour in shapes.iter().flatten() {
        assert!(contour.len() >= 3, "seed={seed} contour={contour:?}");
        assert!(
            contour
                .iter()
                .all(|point| point[0].is_finite() && point[1].is_finite()),
            "seed={seed} contour contains a non-finite point: {contour:?}"
        );
    }
}

struct StressRng {
    state: u64,
}

impl StressRng {
    fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0xa076_1d64_78bd_642f,
        }
    }

    fn range_usize(&mut self, min: usize, max: usize) -> usize {
        min + self.next_u32() as usize % (max - min + 1)
    }

    fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_u32() as f32 / u32::MAX as f32
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(0xe703_7ed1_a0b4_28db)
            .wrapping_add(0x8ebc_6af0_9c88_c6e3);
        (self.state >> 32) as u32
    }
}

fn next_stress_seed(seed: u64) -> u64 {
    seed.wrapping_mul(0xe703_7ed1_a0b4_28db)
        .wrapping_add(0x8ebc_6af0_9c88_c6e3)
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn panic_message(payload: Box<dyn core::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        return (*message).to_string();
    }
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    "non-string panic payload".to_string()
}
