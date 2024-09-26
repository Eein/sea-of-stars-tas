use super::GuiHelper;

#[derive(Debug)]
pub struct TitleHelper {
    name: String
}
impl Default for TitleHelper {
    fn default() -> TitleHelper {
        Self {
            name: "Title Helper".to_string()
        }
    }

}


impl GuiHelper for TitleHelper {
    fn name(&self) -> String {
        self.name.clone()
    }
}
