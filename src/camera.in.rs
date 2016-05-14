#[derive(Clone, Deserialize, Serialize)]
pub enum CameraAction {
    Move(Direction),
}
