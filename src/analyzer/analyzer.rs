use crate::{
    classifier::classifier::LanguageClassifier, 
    neural_net::yolo::Yolo, 
    parser::{ProtoParser, knowledge_component::KnowledgeComponent}
};
use std::{env::current_dir, error::Error, fs::write, process::Command, str::from_utf8, sync::{Arc, Mutex, mpsc}, thread, time::Duration};
use indexmap::IndexSet;
use serde::{Serialize, Deserialize};


const CLASSIFICATIONTHRESHOLD: usize = 8;

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
        let (sender, receiver) = mpsc::channel::<(String, i32)>();
        let url = self.video.url.clone();

        let mut parser = Arc::new(Mutex::new(ProtoParser::new()));
        let mut parser_clone = parser.clone();
        
        // TODO implement break condition
        let handle = thread::spawn(move || {
            // let mut parser = ProtoParser::new_empty();
            if let Some(classification) = LanguageClassifier::classify(&url) {
                    // let mut parser = ProtoParser::new(&url, classification);
                    parser_clone.lock().unwrap().parse_language(&url, classification);
                    loop {
                        let (value, time_code) = receiver.recv().unwrap();
                        // parser.parse(&value, time_code).unwrap();
                        parser_clone.lock().unwrap().parse(&value, time_code).unwrap();
                    }
            }
            else {
                let mut classification_string = String::new();
                let mut classify = true;
                loop {
                    // TODO: wrap unwrap
                    let (value, time_code) = receiver.recv().unwrap();
                    if classify {
                        classification_string.push_str(&value);
                        if classification_string.chars().count() >= CLASSIFICATIONTHRESHOLD {
                            if let Some(classification) = LanguageClassifier::classify_ml(&value) {
                                // parser = ProtoParser::new(&url, classification);
                                parser_clone.lock().unwrap().parse_language(&url, classification);
                            }
                            classify = false;
                        }
                    }
                    // parser.parse(&value, time_code).unwrap();
                    parser_clone.lock().unwrap().parse(&value, time_code).unwrap();
                    thread::sleep(Duration::from_millis(500));
                }
            }    
        });

        Yolo::run(sender, &self.video.path)?;

        // TODO: implement transfer of knowledge components
        handle.join();
        self.knowledge_components = Arc::try_unwrap(parser).unwrap().into_inner().unwrap().get_knowledge_components();
        // Arc::try_unwrap(parser).unwrap().into_inner().unwrap().parser.unwrap().get_knowledge_components();
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