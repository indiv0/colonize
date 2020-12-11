use bevy::prelude::{AppBuilder, Commands, IntoSystem, Plugin};

struct Dwarf;
struct Name(String);

fn add_dwarves(commands: &mut Commands) {
    commands
        .spawn((Dwarf, Name("Khustrul Lavablade".to_string())))
        .spawn((Dwarf, Name("Vaddul Cavemantle".to_string())))
        .spawn((Dwarf, Name("Ormat Caskfury".to_string())))
        .spawn((Dwarf, Name("Alfodgror Steelbeard".to_string())))
        .spawn((Dwarf, Name("Krorhunri Mithrilshoulder".to_string())))
        .spawn((Dwarf, Name("Noramilin Blazingsunder".to_string())))
        .spawn((Dwarf, Name("Snastaenelyn Whitbow".to_string())))
        .spawn((Dwarf, Name("Kizneabela Horndelver".to_string())))
        .spawn((Dwarf, Name("Korkkalynn Opalback".to_string())))
        .spawn((Dwarf, Name("Thulgreline Oakview".to_string())));
}

pub(crate) struct DwarfPlugin;

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_dwarves.system());
    }
}