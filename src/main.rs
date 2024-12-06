use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_string()));
    let mut naviagtor = Navigator::new(db.clone());

    loop {
        // clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action

        if let Some(page) = naviagtor.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!(
                    "Error rendering page: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            }
            let user_input = get_user_input();
            match page.handle_input(&user_input) {
                Ok(action) => {
                    if let Some(user_action) = action {
                        if let Err(error) = naviagtor.handle_action(user_action) {
                            println!("Error handling processing user input: {}\nPress any key to continue...", error);
                            wait_for_key_press();
                        }
                    }
                },
                Err(error) => {
                    println!(
                        "Error getting user input: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                }
            }
        } else {
            break;
        }
    }
}
