// use std::io;
use std::collections::{HashMap, HashSet};
use std::str::Lines;
use std::fs;

// use std::error::Error;
mod classes;
use self::classes::{Chart, Bar, Note, NoteGroup, Hold};

// Final formatting of the output file
pub fn generate_output_file(chart: &Chart, bars: &Vec<Bar>, output_file: &str) {
    let mut output = String::new();
    output.push_str(&format!("{{\n\t\"chartinfo\": " ));
    output.push_str(&chart.to_string());
    output.push_str(&format!(",\n"));
    output.push_str(&format!("\n\"bars\": [\n"));
    for (i, bar) in bars.iter().enumerate() {
        output.push_str(&format!("\t{}", bar.to_string()));
        if i < bars.len() - 1 {
            output.push_str(",\n");
        } else {
            output.push_str("\n");
        }
    }
    output.push_str(&format!("]\n}}"));
    fs::write(output_file, output).expect("Unable to write file");

    println!("Output file: {}", output_file);
}

//  Sorts and groups notes by time
pub fn sort_and_group_notes(bars: &mut Vec<Bar>, barelements: &mut Vec<Vec<Note>>) {
    for i in 0..barelements.len() {
        barelements[i as usize].sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    for i in 0..barelements.len() {
        let mut j = 0;

        while j < barelements[i as usize].len() {
            let mut k = j;
            let time = barelements[i as usize][j as usize].time;
            let mut group = NoteGroup {
                time: barelements[i as usize][j as usize].time,
                channels: HashSet::<u32>::new(),
                holds: HashSet::<Option<Hold>>::new(),
            };

            while k < barelements[i as usize].len() && barelements[i as usize][k as usize].time == time {
                if barelements[i as usize][k as usize].channel <= 20 {
                    group.channels.insert(barelements[i as usize][k as usize].channel);
                } else {
                    group.holds.insert(barelements[i as usize][k as usize].hold);
                }
                k += 1;
            }

            j = k;
            bars[i as usize].notes.push(group);
        }
    }
}

// ##########################################
// # Functions that process the chart data
// ##########################################

pub fn process_chart_info(lines: &mut std::str::Lines) -> (HashMap<String, f64>, HashMap<String, u32>, Chart) {
    let mut bpms: HashMap<String, f64> = HashMap::new();
    let mut stops: HashMap<String, u32> = HashMap::new();
    let mut chart = Chart {
        genre: String::new(),
        title: String::new(),
        artist: String::new(),
        bpm: 0.0,
        playlevel: 0,
        rank: 0,
        subtitle: String::new(),
    };

    let mut line = lines.next();
    let mut line_value = String::new();

    while line.is_none() == false && line_value.starts_with("*---------------------- MAIN DATA FIELD") == false {
        line_value = line.unwrap().to_string();

        let chart_info = line_value[..].split(" ").collect::<Vec<&str>>();

        let info = chart_info[0].to_string();
        let info_value = chart_info[1..].join(" ").to_string();
        if chart_info[0] == "#GENRE" {
            chart.genre = info_value.to_string();
        } else if chart_info[0] == "#TITLE" {
            chart.title = info_value.to_string();
        } else if chart_info[0] == "#ARTIST" {
            chart.artist = info_value.to_string();
        } else if chart_info[0] == "#BPM" {
            chart.bpm = chart_info[1].parse().unwrap();
        } else if chart_info[0] == "#PLAYLEVEL" {
            chart.playlevel = chart_info[1].parse().unwrap();
        } else if chart_info[0] == "#RANK" {
            chart.rank = chart_info[1].parse().unwrap();
        } else if chart_info[0] == "#SUBTITLE" {
            chart.subtitle = info_value.to_string();
        } else if chart_info[0].starts_with("#BPM") {
            bpms.insert(info[4..].to_string(), info_value.parse().unwrap());
        } else if chart_info[0].starts_with("#STOP") {
            stops.insert(info[5..].to_string(), info_value.parse().unwrap());
        }
        line = lines.next();
    }

    (bpms, stops, chart)
}

const MAXLEN: f64 = 1000.0;

pub fn process_main_data(main_start: &mut Lines, chart: &Chart, bpms: &HashMap<String, f64>, stops: &HashMap<String, u32>) -> (Vec<Bar>, Vec<Vec<Note>>) {
    let mut bars = Vec::<Bar>::new();
    let mut barelements = Vec::<Vec<Note>>::new();
    let bpm = chart.bpm;
    let mut holdstart = [0.0; 8];

    let mut nbars = 0;
    let mut lines = main_start.clone();

    let mut line = lines.next();
    let mut line_value;

    while line.is_none() == false {
        line_value = line.unwrap().to_string();
        if line_value.starts_with("#") == true {
            nbars = line_value[1..4].parse::<u32>().unwrap();
        }
        line = lines.next();
    }
    nbars += 1;

    lines = main_start.clone();
    line = lines.next();

    for _ in 0..nbars {
        bars.push(Bar::default());
        barelements.push(Vec::<Note>::new());
    }

    while line.is_none() == false {
        line_value = line.unwrap().to_string();

        if !line_value.starts_with("#") {
            line = lines.next();
            continue;
        }

        let barnumber = line_value[1..4].parse::<u32>().unwrap();
        let channel = line_value[4..6].parse::<u32>().unwrap();
        let value = line_value[7..].to_string();


        bars[barnumber as usize].bpmvalue = bpm;

        if channel == 2 {
            process_sigchange(&mut bars, barnumber, value);
        } else if channel == 3 {
            process_bpmchange(&mut bars, barnumber, value, &chart);
        } else if channel == 8 {
            process_bpmchange2(&mut bars, barnumber, value, &bpms);
        } else if channel == 9 {
            process_stop(&mut bars, barnumber, value, &stops);
        } else
        if (channel >= 11 && channel <= 19) || (channel >= 51 && channel <= 59) {
            process_note(&mut barelements, &mut holdstart, barnumber, channel, value, &bars, &chart);
        }

        line = lines.next();
    }

    for i in 0..barelements.len() {
        barelements[i as usize].sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    (bars, barelements)
}


pub fn process_note(
    barelements: &mut Vec<Vec<Note>>,
    holdstart: &mut [f64; 8],
    barnumber: u32,
    mut channel: u32,
    value: String,
    bars: &Vec<Bar>,
    chart: &Chart,
) {
    // channel -= 1;

    match channel % 10 {
        7 | 8 | 9 => {
            channel -= 2;
        }
        _ => {}
    }

    let objects = value.chars().collect::<Vec<char>>();
    let objects = objects.chunks(2);
    let objects = objects.map(|x| x.iter().collect::<String>()).collect::<Vec<String>>();

    for j in 0..objects.len() {
        let object = objects[j].to_string();
        let mut hold = <Option<Hold>>::None;
        let mut starttime: f64 = j as f64 / objects.len() as f64;

        if object == "00" {
            continue;
        }

        // check if it's a hold
        if channel / 10 == 5 {
            // check if it's the start of the hold
            if holdstart[(channel % 10) as usize] == 0.0 {
                holdstart[(channel % 10) as usize] = barnumber as f64 + starttime;
            } else {
                let mut hold_unwrap = Hold {
                    channel: channel,
                    start: holdstart[(channel % 10) as usize] % 1.0,
                    length: 0.0,
                };
                
                let mut tmpbpm = bars[holdstart[(channel % 10) as usize].floor() as usize].bpmvalue;
                let startlen = bars[holdstart[(channel % 10) as usize].floor() as usize].sigvalue * hold_unwrap.start * chart.bpm / bars[holdstart[(channel % 10) as usize].floor() as usize].bpmvalue;

                for k in holdstart[(channel % 10) as usize].floor() as usize .. barnumber as usize {
                    if bars[k].bpmchange == true {
                        tmpbpm = bars[k].bpmvalue;
                    }
                    hold_unwrap.length += bars[k].sigvalue * chart.bpm / tmpbpm;
                }
                hold_unwrap.length += j as f64 / objects.len() as f64 * bars[barnumber as usize].sigvalue * chart.bpm / bars[barnumber as usize].bpmvalue - startlen;
                
                if hold_unwrap.length > MAXLEN {
                    hold_unwrap.length = MAXLEN;
                }

                holdstart[(channel % 10) as usize] = 0.0;

                starttime = hold_unwrap.start;
                hold = Some(hold_unwrap);
            }
        }

        barelements[barnumber as usize].push(Note {
            channel: channel,
            object: object,
            hold: hold,
            time: starttime as f64,
        });
    }
}

pub fn process_sigchange(bars: &mut Vec<Bar>, barnumber: u32, value: String) {
    bars[barnumber as usize].sigchange = true;
    bars[barnumber as usize].sigvalue = value.parse::<f64>().unwrap();
}

pub fn process_bpmchange(bars: &mut Vec<Bar>, barnumber: u32, value: String, chart: &Chart) {
    bars[barnumber as usize].bpmchange = true;
    let bpmvalues = value.chars().collect::<Vec<char>>();
    let bpmvalues = bpmvalues.chunks(2);
    let mut index: u32 = 0;
    let size = bpmvalues.len();

    for bpmvalue in bpmvalues {
        let bpmvalue = bpmvalue.iter().collect::<String>();
        if bpmvalue != "00" {
            bars[barnumber as usize].bpmvalue = u32::from_str_radix(&bpmvalue, 16).unwrap() as f64;
            break;
        }
        index += 1;
    }

    bars[barnumber as usize].sigvalue *= ((size as u32 - index) as f64 + index as f64 * bars[barnumber as usize].bpmvalue as f64 / chart.bpm as f64) / size as f64;
}

pub fn process_bpmchange2(bars: &mut Vec<Bar>, barnumber: u32, value: String, bpms: &HashMap<String, f64>) {
    bars[barnumber as usize].bpmchange = true;
    bars[barnumber as usize].bpmvalue = bpms[&value];
}

pub fn process_stop(bars: &mut Vec<Bar>, barnumber: u32, value: String, stops: &HashMap<String, u32>) {
    bars[barnumber as usize].stop = true;
    bars[barnumber as usize].stopvalue = stops[&value];
}
