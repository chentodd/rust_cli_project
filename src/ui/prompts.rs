use crate::{
    io_utils::get_user_input,
    models::{Epic, Status, Story},
};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("----------------------------");
    println!("Epic Name: ");

    let name = get_user_input().trim().to_owned();

    println!("Epic Description: ");

    let desc = get_user_input().trim().to_owned();

    Epic::new(name, desc)
}

fn create_story_prompt() -> Story {
    println!("----------------------------");
    println!("Story  Name: ");

    let name = get_user_input().trim().to_owned();

    println!("Story  Description: ");

    let desc = get_user_input().trim().to_owned();

    Story::new(name, desc)
}

fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]: ");

    let input = get_user_input().trim().to_owned();
    if input == "Y" {
        return true;
    }

    false
}

fn delete_story_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this story? [Y/n]: ");

    let input = get_user_input().trim().to_owned();
    if input == "Y" {
        return true;
    }

    false
}

fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED): ");

    let status = get_user_input();
    let status = status.trim().parse::<u8>();

    if let Ok(cmd) = status {
        match cmd {
            1 => return Some(Status::Open),
            2 => return Some(Status::InProgress),
            3 => return Some(Status::Resolved),
            4 => return Some(Status::Closed),
            _ => return None,
        }
    }

    None
}
