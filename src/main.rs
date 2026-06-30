pub mod application;
pub mod engine;
pub mod liminal;
pub mod render;
pub mod visual;
pub mod terrain;
pub mod world;

fn main() -> anyhow::Result<()>
{
     application::run::<liminal::Liminal>()?;
     Ok(())
}
