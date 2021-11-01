use crate::{
    classifier::classifier::LanguageClassifier, 
    neural_net::yolo::Yolo, 
    parser::{ProtoParser, knowledge_component::KnowledgeComponent}
};
use std::{env::current_dir, error::Error, fs::write, process::Command, str::from_utf8, sync::{Arc, Mutex, mpsc::{self, Receiver}}, thread, time::Duration};
use indexmap::IndexSet;
use serde::{Serialize, Deserialize};

const CLASSIFICATION_THRESHOLD: usize = 8;

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
        self.download_video()?;
        let (sender, receiver) = mpsc::channel::<(Message, i32)>();
        let url = self.video.url.clone();

        let handle = thread::spawn(move || {
            parse_knowledge_components(receiver, url)
        });
        
        Yolo::run(sender, &self.video.path)?;

        let parser = handle.join().expect("Error joining handle");
        self.knowledge_components = parser.get_knowledge_components();
        
        Ok(())
    }

    pub fn download_video(&self) -> Result<(), Box<dyn Error>> {
        Command::new("youtube-dl").args(&["-f", "best", "-o", &format!("video//{}.mp4", self.video.title), &self.video.url]).output()?;
        Ok(())
    }

    pub fn save_result(&self) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string_pretty(&self)?;
        std::fs::create_dir("./output");
        write(format!("./output/{}.json", self.video.title), serialized)?;
        
        Ok(())
    }
}

fn parse_knowledge_components(receiver: Receiver<(Message, i32)>, url: String) -> ProtoParser {
    let mut parser = ProtoParser::new();
    
    if let Some(classification) = LanguageClassifier::classify(&url) {
            parser.parse_language(&url, classification);
            
            loop {
                if let Ok(message) = receiver.recv() {
                    match message.0 {
                        Message::StreamMessage(msg) => {
                            parser.parse(&msg, message.1).unwrap();
                        },
                        Message::EndMessage => break,
                    }
                }
                thread::sleep(Duration::from_millis(500));
            }
    }
    else {
        let mut classification_string = String::new();
        let mut classify = true;
        
        loop {
            if let Ok(message) = receiver.recv() {
                match message.0 {
                    Message::StreamMessage(msg) => {
                        if classify {
                            classification_string.push_str(&msg);
                            
                            if classification_string.chars().count() >= CLASSIFICATION_THRESHOLD {
                                if let Some(classification) = LanguageClassifier::classify_ml(&msg) {
                                    parser.parse_language(&url, classification);
                                }
                                classify = false;
                            }
                        }
                        parser.parse(&msg, message.1).unwrap();
                    },
                    Message::EndMessage => break,
                }
            }  
            thread::sleep(Duration::from_millis(500));
        }
    }   
    parser
}

pub enum Message {
    StreamMessage(String),
    EndMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub title: String,
    pub url: String,
    #[serde(skip)]
    pub path: String
}

impl Video {
    fn new(url: &str) -> Self {
        let command_output = Command::new("youtube-dl").args(&["--get-title", &url]).output().expect("Error: youtube-dl could not get video title.");
        let mut video_title = from_utf8(&command_output.stdout).unwrap().to_owned();
        let path = format!("{}/video/{}.mp4", current_dir().unwrap().display().to_string(), video_title);

        if video_title.ends_with("\n") {
            video_title.pop();
        }

        Self {
            title: video_title,
            url: url.into(),
            path: path
        }
    }
}