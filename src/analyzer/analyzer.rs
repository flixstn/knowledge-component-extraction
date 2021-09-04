use std::{process::Command, str::from_utf8, fs::write, error::Error};
use indexmap::IndexSet;
use serde::{Serialize, Deserialize};

use crate::{neural_net::yolo::Yolo, parser::{parser::Parser, knowledge_component::KnowledgeComponent}};

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoAnalyzer {
    pub video: Video,
    pub language: String,
    #[serde(rename = "knowledgeComponents")]
    pub knowledge_components: IndexSet<KnowledgeComponent>
}

impl VideoAnalyzer {
    pub fn new(url: &str) -> Self {
        Self {
            video: Video::new(url),
            language: String::new(),
            knowledge_components: IndexSet::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.download()?;
        let mut parser = Parser::new();
        Yolo::run(&mut parser)?;

        self.knowledge_components = parser.knowledge_components;

        Ok(())
    }

    pub fn download(&self) -> Result<(), Box<dyn Error>> {
        Command::new("youtube-dl").args(&["-f", "best", "-o", &format!("video//{}.mp4", self.video.title), &self.video.url]).output()?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string_pretty(&self)?;
        write(format!("./src/{}.json", self.video.title), serialized)?;
        
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    title: String,
    url: String,
}

impl Video {
    fn new(url: &str) -> Self {
        let command_output = Command::new("youtube-dl").args(&["--get-title", &url]).output().expect("Error: youtube-dl could not get video title.");
        let mut video_title = from_utf8(&command_output.stdout).unwrap().to_owned();

        if video_title.ends_with("\n") {
            video_title.pop();
        }

        Self {
            title: video_title,
            url: url.into()
        }
    }
}