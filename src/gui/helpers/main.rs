use super::GuiHelper;

#[derive(Debug)]
pub struct MainHelper {
    name: String
}
impl Default for MainHelper {
    fn default() -> MainHelper {
        Self {
            name: "Main".to_string()
        }
    }

}


impl GuiHelper for MainHelper {
    fn name(&self) -> String {
        self.name.clone()
    }
}
