use std::path::Path;
use stereokit_rs::constants::VEC2_ONE;
use stereokit_rs::enums::RenderLayer;
use stereokit_rs::enums::TextAlign::TopLeft;
use stereokit_rs::functions::{sk_init, sk_run_data};
use stereokit_rs::material::Material;
use stereokit_rs::mesh::Mesh;
use stereokit_rs::model::Model;
use stereokit_rs::pose::PoseIdentity;
use stereokit_rs::texture::Texture;
use stereokit_rs::ui::window::MoveType::MoveExact;
use stereokit_rs::ui::window::WindowType::WindowNormal;
use stereokit_rs::values::{Color128, Color32, Vec2, Vec3};
use stereokit_sys::sk_settings_t;
use stereokit_rs::ui::{text, window};
pub fn main() {
    if !sk_init(stereokit_macros::create_sk_settings!()) {
        return;
    }

    let mut test_closure = || {
        window::begin("My Window", PoseIdentity, Vec2::from([1.0f32, 1.0f32]), WindowNormal, MoveExact);
        text::create("test_text", TopLeft);
        window::end();
    };
    let my_closure: &mut dyn FnMut() = &mut test_closure;
    sk_run_data(&mut Box::new(my_closure), &mut Box::new(&mut || { println!("hello world!")}));
}