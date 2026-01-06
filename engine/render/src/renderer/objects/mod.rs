use glam::Mat4;

pub trait RenderObject {
    fn model_matrix(&self) -> Mat4;
    fn is_floor(&self) -> bool;
}
