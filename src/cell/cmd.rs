#[derive(Debug, Clone, Copy)]
pub enum Command {
    Body(CommandBody),
}

#[derive(Debug, Clone, Copy)]
pub enum CommandBody {
    Glycolysis,
    Photosynthesis,
}
