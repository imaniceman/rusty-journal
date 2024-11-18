// use anyhow::Ok;
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::from_reader;
use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
    #[serde(
        serialize_with = "serialize_optional_datetime",
        deserialize_with = "deserialize_optional_datetime",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub completed_at: Option<DateTime<Utc>>,
}
fn serialize_optional_datetime<S>(
    date: &Option<DateTime<Utc>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => ts_seconds::serialize(d, serializer),
        None => serializer.serialize_none(),
    }
}

fn deserialize_optional_datetime<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<i64> = Option::deserialize(deserializer)?;
    match opt {
        Some(ts) => {
            ts_seconds::deserialize(serde::de::IntoDeserializer::into_deserializer(ts)).map(Some)
        }
        None => Ok(None),
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let created_at = self.create_at.with_timezone(&Local).format("%F %H:%M:%S");
        let complete_at = self
            .completed_at
            .map(|c| c.with_timezone(&Local).format("%F %H:%M:%S").to_string());
        let text_width = UnicodeWidthStr::width(self.text.as_str());
        let padding = if text_width < 80 { 80 - text_width } else { 0 };

        let padded_text = format!("{}{}", self.text, " ".repeat(padding));
        match complete_at {
            Some(_) => write!(
                f,
                "{} [{}] (completed at {})",
                padded_text,
                created_at,
                complete_at.unwrap()
            ),
            None => write!(f, "{} [{}]", padded_text, created_at),
        }
    }
}

impl Task {
    pub fn new(text: String) -> Task {
        let create_at = Utc::now();
        Task {
            text,
            create_at,
            completed_at: None,
        }
    }
}

fn collect_task(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks: Vec<Task> = match from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_task(&file)?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;
    let mut tasks = collect_task(&file)?;
    let mut incomplete_tasks: Vec<_> = tasks
        .iter_mut()
        .filter(|t| t.completed_at.is_none())
        .collect();
    if task_position == 0 || task_position > incomplete_tasks.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid task position",
        ));
    }
    incomplete_tasks[task_position - 1].completed_at = Some(Utc::now());

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = collect_task(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks.iter().filter(|t| t.completed_at.is_none()) {
            println!("{}. {}", order, task);
            order += 1;
        }
    }
    Ok(())
}
pub fn list_completed_tasks(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = collect_task(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks.iter().filter(|t| t.completed_at.is_some()) {
            println!("{}. {}", order, task);
            order += 1;
        }
    }
    Ok(())
}
pub fn edit_task(journal_path: PathBuf, task_position: usize, text: String) -> Result<()> {
    // 只能修改未完成的任务，而且 task_position 需要排除已完成的任务
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_task(&file)?;
    let mut incomplete_tasks: Vec<_> = tasks
        .iter_mut()
        .filter(|t| t.completed_at.is_none())
        .collect();
    if task_position == 0 || task_position > incomplete_tasks.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid task position",
        ));
    };
    incomplete_tasks[task_position - 1].text = text;
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_list_completed_tasks() -> Result<()> {
        // Create a temporary directory
        let dir = tempdir()?;
        let file_path = dir.path().join("test_journal.json");

        // Create a sample task list with one completed task
        let tasks = vec![
            Task {
                text: String::from("Task 1"),
                create_at: Utc::now(),
                completed_at: Some(Utc::now()),
            },
            Task {
                text: String::from("Task 2"),
                create_at: Utc::now(),
                completed_at: None,
            },
        ];

        // Write the tasks to the file
        let file = File::create(&file_path)?;
        serde_json::to_writer(&file, &tasks)?;
        file.sync_all()?;

        list_tasks(file_path.clone())?;

        // Call the function to list completed tasks
        list_completed_tasks(file_path.clone())?;

        // Clean up
        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_complete_task() -> Result<()> {
        // Create a temporary directory
        let dir = tempdir()?;
        let file_path = dir.path().join("test_journal.json");

        // Create a sample task list with one incomplete task
        let tasks = vec![
            Task {
                text: String::from("Task 1"),
                create_at: Utc::now(),
                completed_at: None,
            },
            Task {
                text: String::from("Task 2"),
                create_at: Utc::now(),
                completed_at: None,
            },
            Task {
                text: String::from("Task 3"),
                create_at: Utc::now(),
                completed_at: Some(Utc::now()),
            },
            Task {
                text: String::from("Task 4"),
                create_at: Utc::now(),
                completed_at: None,
            },
        ];

        // Write the tasks to the file
        let file = File::create(&file_path)?;
        serde_json::to_writer(&file, &tasks)?;
        file.sync_all()?;

        list_tasks(file_path.clone())?;
        println!("Complete task 1");
        // Complete the first task
        complete_task(file_path.clone(), 1)?;

        // Read the tasks back from the file
        let file = File::open(&file_path)?;
        let tasks: Vec<Task> = from_reader(file)?;

        // Check that the first task is completed
        assert!(tasks[0].completed_at.is_some());
        assert!(tasks[1].completed_at.is_none());

        list_tasks(file_path.clone())?;
        println!("Complete task 4");
        complete_task(file_path.clone(), 2)?;
        println!("Left");
        list_tasks(file_path.clone())?;

        println!("Already complete ----");
        list_completed_tasks(file_path.clone())?;

        let file = File::open(&file_path)?;
        let tasks: Vec<Task> = from_reader(file)?;

        assert_eq!(tasks.len(), 4);
        assert!(tasks[0].completed_at.is_some());
        assert!(tasks[1].completed_at.is_none());
        assert!(tasks[2].completed_at.is_some());
        assert!(tasks[3].completed_at.is_some());

        // Clean up
        dir.close()?;
        Ok(())
    }
}
