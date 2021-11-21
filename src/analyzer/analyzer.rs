use crate::prelude::*;

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
        self.language = parser.language.unwrap().to_string().to_lowercase();
        
        Ok(())
    }

    pub fn download_video(&self) -> Result<(), Box<dyn Error>> {
        Command::new("yt-dlp")
            .args(&["-f", "248", "-P", &format!("video//"), "-o", &format!("{}.mp4", self.video.title) , &format!("{}", self.video.url)]).output()?;

        Ok(())
    }

    pub fn save_result(&self) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string_pretty(&self)?;
        create_dir("./output")?;
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
        let video_title = get_video_title(url);
        let path = format!("{}/video/{}.mp4", current_dir().unwrap().display().to_string(), video_title);

        Self {
            title: video_title,
            url: url.into(),
            path: path
        }
    }
}

fn get_video_title(url: &str) -> String {
    let command_output = Command::new("yt-dlp").args(&["--print", "title", &format!("{}", url)]).output().expect("Error: youtube-dl could not get video title.");
    let mut video_title = from_utf8(&command_output.stdout).unwrap().to_owned();

    if video_title.ends_with("\n") {
        video_title.pop();
    }

    video_title
}