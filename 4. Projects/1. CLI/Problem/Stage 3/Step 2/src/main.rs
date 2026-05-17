use std::process::exit;
use std::rc::Rc;
use anyhow::{anyhow, Context};

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // TODO: create database and navigator
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_string()));
    let mut navigator = Navigator::new(db);
    
    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action

        let current_page = match navigator.get_current_page() {
            None => break,
            Some(cp) => cp,
        };

        if let Err(e) =  current_page.draw_page() {
            println!("An error happened on page drawing: {}", e);
            wait_for_key_press();
            break;
        }

        match current_page.handle_input(&get_user_input()) {
            Err(e) => {
                println!("An error happened on the input handling: {}", e);
                wait_for_key_press();
                break;
            },
            Ok(None) => {
                println!("No actions found by the input");
                wait_for_key_press();
                break;
            },
            Ok(Some(action)) => {
                if let Err(e) =  navigator.handle_action(action.clone()) {
                    println!("Action handling error: {}", e);
                    exit(1);
                }
            }
        }
    }
}
