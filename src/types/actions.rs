use types::resources::Resource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    AddMessage(String),
    //SetBoolFlag(BoolFlag),
    //ClearBoolFlag(BoolFlag),
    SetResourceValue(Resource, i32),
    AddResourceValue(Resource, i32),
    //AddIntFlag(IntFlag, i32),
    //SetFloatFlag(FloatFlag),
}

// TODO timeactions which fire at a given tick - can limp in with Action
