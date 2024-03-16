use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};
pub struct MenuBuilder {}
// impl MenuBuilder {
    // const MENU: Vec<MenuItemBuilder> = vec![
    //     MenuItemBuilder::new1(
    //         "File",
    //         "File",
    //         vec![
    //             MenuItemBuilder::new("Copy", "Copy"),
    //             MenuItemBuilder::new("Paste", "Paste"),
    //             MenuItemBuilder::new("Cut", "Cut"),
    //             MenuItemBuilder::new("Paste", "Paste"),
    //         ],
    //     ),
    //     MenuItemBuilder::new1(
    //         "Setting",
    //         "Setting",
    //         vec![
    //             MenuItemBuilder::new("About", "About"),
    //             MenuItemBuilder::new("Exit", "Exit"),
    //         ],
    //     ),
    // ];
// }

// struct MenuItemBuilder {
//     id: String,
//     title: String,
//     children: Option<Vec<MenuItemBuilder>>,
// }

// impl MenuItemBuilder {
    // fn new(id: &str, title: &str) -> Self {
    //     MenuItemBuilder {
    //         id: id.to_string(),
    //         title: title.to_string(),
    //         children: None,
    //     }
    // }

    // fn new1(id: &str, title: &str, children: Vec<MenuItemBuilder>) -> Self {
    //     MenuItemBuilder {
    //         id: id.to_string(),
    //         title: title.to_string(),
    //         children: Some(children),
    //     }
    // }
// }

impl MenuBuilder {
    pub fn new() -> Menu {
        // let structure: Vec<RowMenuItem> = vec![];
        let quit = CustomMenuItem::new("quit".to_string(), "Quit");
        let close = CustomMenuItem::new("close".to_string(), "Close");
        let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
        let menu = Menu::new()
            .add_native_item(MenuItem::Copy)
            .add_item(CustomMenuItem::new("hide", "Hide"))
            .add_submenu(submenu);
        menu
    }

    pub fn on_click(event: WindowMenuEvent) {
        match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        }
    }
}
