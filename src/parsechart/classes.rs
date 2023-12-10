// Chart (genre, title, artist, bpm, playlevel, rank, subtitle)
// example of Chart information
// #GENRE MUSIC
// #TITLE Agito
// #ARTIST polycube
// #BPM 172
// #PLAYLEVEL 3
// #RANK 3

// #SUBTITLE

use std::collections::{HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

// implement to string in a json format

// Chart structure that contains all the information of a chart
pub struct Chart {
    pub genre: String,
    pub title: String,
    pub artist: String,
    pub bpm: f64,
    pub playlevel: u32,
    pub rank: u32,
    pub subtitle: String,
}
#[allow(dead_code)]
impl Chart {
    pub fn display(&self){
        print!(
            "Genre: {}\nTitle: {}\nArtist: {}\nBPM: {}\nPlaylevel: {}\nRank: {}\nSubtitle: {}\n", 

            self.genre, self.title, self.artist, self.bpm, self.playlevel, self.rank, self.subtitle
        )
    }
    pub fn to_string(&self) -> String {
        format!("{{\n\t\"genre\": \"{}\",\n\t\"title\": \"{}\",\n\t\"artist\": \"{}\",\n\t\"bpm\": {},\n\t\"playlevel\": {},\n\t\"rank\": {},\n\t\"subtitle\": \"{}\"\n}}", 
            self.genre, self.title, self.artist, self.bpm, self.playlevel, self.rank, self.subtitle
        )
    }
}

// Bar (notes, sigchange, sigvalue, bpmchange, bpmvalue, stop, stopvalue)
// - notes order list of note objects
// - sigchange true if signature change
// - sigvalue signature value default 1
// - bpmchange true if bpm change
// - bpmvalue bpm value default 0
// - stop true if stop default false
// - stopvalue stop value default 0

// Bar structure that contains all the information of a bar
// implement the default values


pub struct Bar {
    pub notes: Vec<NoteGroup>,
    pub sigchange: bool,
    pub sigvalue: f64,
    pub bpmchange: bool,
    pub bpmvalue: f64,
    pub stop: bool,
    pub stopvalue: u32,
}
#[allow(dead_code)]
impl Bar {
    pub fn display(&self){
        print!(
            "Notes: {}\nSigchange: {}\nSigvalue: {}\nBpmchange: {}\nBpmvalue: {}\nStop: {}\nStopvalue: {}\n", 

            self.notes.len(), self.sigchange, self.sigvalue, self.bpmchange, self.bpmvalue, self.stop, self.stopvalue
        )
    }

    pub fn to_string(&self) -> String {
        let mut notes = String::new();
        notes.push_str("[");
        for (index, note) in self.notes.iter().enumerate() {
            notes.push_str(&format!("{}", &note.to_string()));
            if index < self.notes.len() - 1 {
                notes.push_str(",\n");
            }
        }
        notes.push_str("]");
        format!("{{\n\t\"notes\": {},\n\t\"sigchange\": {},\n\t\"sigvalue\": {},\n\t\"bpmchange\": {},\n\t\"bpmvalue\": {},\n\t\"stop\": {},\n\t\"stopvalue\": {}\n}}", 
            notes, self.sigchange, self.sigvalue, self.bpmchange, self.bpmvalue, self.stop, self.stopvalue
        )
    }
}
impl Default for Bar {
    fn default() -> Self {
        Bar {
            notes: Vec::<NoteGroup>::new(),
            sigchange: false,
            sigvalue: 1.0,
            bpmchange: false,
            bpmvalue: 0.0,
            stop: false,
            stopvalue: 0,
        }
    }
}
// Note (channel, object, hold, time)
// - channel channel number
// - object object string (2 characters)
// - hold union null or hold object
// - time when the note is played (int)

// Note structure that contains all the information of a note

pub struct Note {
    pub channel: u32,
    pub object: String,
    pub hold: Option<Hold>,
    pub time: f64,
}
#[allow(dead_code)]
impl Note {
    pub fn display(&self){
        print!(
            "Channel: {}\nObject: {}\nHold: {}\nTime: {}\n", 

            self.channel, self.object, self.hold.is_some(), self.time
        )
    }

    pub fn to_string(&self) -> String {
        let mut hold = String::new();
        if self.hold.is_some() {
            hold = self.hold.unwrap().to_string();
        }
        format!("{{\n\t\"channel\": {},\n\t\"object\": \"{}\",\n\t\"hold\": {},\n\t\"time\": {}\n}}", 
            self.channel, self.object, hold, self.time
        )
    }
}
impl Default for Note {
    fn default() -> Self {
        Note {
            channel: 0,
            object: String::new(),
            hold: None,
            time: 0.0,
        }
    }
}
// Hold (channel, start, length)
// - channel channel number
// - start start time of the hold
// - length length of the hold

// Hold structure that contains all the information of a hold
#[derive(Copy, Clone)]
pub struct Hold {
    pub channel: u32,
    pub start: f64,
    pub length: f64,
}
impl PartialEq for Hold {
    fn eq(&self, other: &Self) -> bool {
        self.channel == other.channel && self.start == other.start && self.length == other.length
    }
}
impl Eq for Hold {}
impl Hash for Hold {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.channel.hash(state);
        self.start.to_bits().hash(state);
        self.length.to_bits().hash(state);
    }
}

impl fmt::Debug for Hold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hold {{ channel: {}, start: {}, length: {} }}", self.channel, self.start, self.length)
    }
}
#[allow(dead_code)]
impl Hold {
    pub fn display(&self){
        print!(
            "Channel: {}\nStart: {}\nLength: {}\n", 

            self.channel, self.start, self.length
        )
    }

    pub fn to_string(&self) -> String {
        format!("{{\n\t\"channel\": {},\n\t\"start\": {},\n\t\"length\": {}\n}}", 
            self.channel, self.start, self.length
        )
    }
}

// NoteGroup (time, channels, holds)
// - time time of the note group
// - channels set of channels
// - holds set of holds
pub struct NoteGroup {
    pub time: f64,
    pub channels: HashSet<u32>,
    pub holds: HashSet<Option<Hold>>
}
#[allow(dead_code)]
impl NoteGroup {
    pub fn display(&self){
        print!(
            "Time: {}\nChannels: {:?}\nHolds: {:?}\n", 

            self.time, self.channels, self.holds
        )
    }

    pub fn to_string(&self) -> String {
        let mut channels = String::new();
        channels.push_str("[");
        for (i, channel) in self.channels.iter().enumerate() {
            channels.push_str(&format!("\"{}\"",&channel.to_string()));
            if i < self.channels.len() - 1 {
                channels.push_str(", ");
            }
        }
        channels.push_str("]");

        let mut holds = String::new();
        holds.push_str("[");
        for (i, hold) in self.holds.iter().enumerate() {
            if hold.is_some() {
                holds.push_str(&hold.unwrap().to_string());
            } else {
                continue
            }
            if i < self.holds.len() - 1 {
                holds.push_str(", ");
            }
        }
        holds.push_str("]");
        format!("{{\n\t\"time\": {},\n\t\"channels\": {},\n\t\"holds\": {}\n}}", 
            self.time, channels, holds
        )
    }
}
