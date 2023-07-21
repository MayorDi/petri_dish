use derive_new::new;

#[derive(new, Debug, Clone, Copy)]
pub struct Environment {
    pub nutritional: f32
}