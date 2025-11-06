use dialoguer::Input;

pub fn prompt_input(prompt: &str) -> String {
    Input::new().with_prompt(prompt).interact_text().unwrap()
}
