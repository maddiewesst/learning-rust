use chrono::{DateTime, Datelike}; // Importing DateTime and Datelike structs from the chrono library
use json::JsonValue; // Importing the JsonValue enum from the json crate
use std::collections::HashMap; // Importing the HashMap struct from the standard library

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct CommitData {
    author: String,
    week: String,
}

fn parse_commit_data(data: &JsonValue) -> Option<CommitData> {
    let author = data["author"]["login"].as_str()?.to_owned(); // Accessing the "login" field of the "author" object of the "data" JsonValue and converting it to an owned String
    let timestamp_str = data["commit"]["author"]["date"].as_str()?; // Accessing the "date" field of the "author" object of the "commit" object of the "data" JsonValue
    let timestamp = DateTime::parse_from_rfc3339(timestamp_str).ok()?; // Parsing the timestamp string into a DateTime using the RFC 3339 format and returning None if parsing fails
    let week = format!("{}-W{}", timestamp.year(), timestamp.iso_week().week()); // Creating a formatted string representing the year and ISO week number of the timestamp

    Some(CommitData { author, week }) // Wrapping the CommitData struct in an Option and returning it
}

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_per_week: HashMap<String, u32> = HashMap::new(); // Creating a new HashMap with keys of type String and values of type u32

    for commit in data.members() { // Looping over the JsonValue objects in the "data" JsonValue
        if let Some(commit_data) = parse_commit_data(commit) { // Parsing the commit data and storing it in an optional CommitData struct
            let count = commits_per_week.entry(commit_data.week).or_insert(0); // Attempting to insert a new key-value pair into the HashMap with a default value of 0 if the key does not exist and returning a mutable reference to the value
            *count += 1; // Incrementing the value of the mutable reference to the value
        }
    }

    commits_per_week // Returning the HashMap
}

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_per_author: HashMap<String, u32> = HashMap::new(); // Creating a new HashMap with keys of type String and values of type u32

    for commit in data.members() { // Looping over the JsonValue objects in the "data" JsonValue
        if let Some(author) = commit["author"]["login"].as_str() { // Accessing the "login" field of the "author" object of the "commit" object of the current JsonValue and storing it in an optional String
            let count = commits_per_author.entry(author.to_owned()).or_insert(0); // Attempting to insert a new key-value pair into the HashMap with a default value of 0 if the key does not exist and returning a mutable reference to the value
            *count += 1; // Incrementing the value of the mutable reference to the value
        }
    }

    commits_per_author // Returning the HashMap
}