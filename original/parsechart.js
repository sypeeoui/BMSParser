/* Script for reading the bms file */

let chartinfo;
const fr = new FileReader();
let bars = [];
const maxlen = 10000;
// example of Chart information
// #GENRE MUSIC
// #TITLE Agito
// #ARTIST polycube ~ øc(alpha complex)
// #BPM 172
// #PLAYLEVEL 3
// #RANK 3
class Chart{
    constructor(){
        this.genre = "";
        this.title = "";
        this.artist = "";
        this.bpm = 0;
        this.playlevel = 0;
        this.rank = 0;
        this.maxbeat = 0;
        this.notecount = 0;
    }
}
class Bar{
    constructor(){
        this.notes = []; // an order list of the groups of notes
        // with the respective channels to play them and time to play them
        this.sigchange = false;
        this.sigvalue = 1; // 4/4
        this.bpmchange = false;
        this.bpmvalue = 0;
        this.stop = false;
        this.stopvalue = 0;
    }
}
class Hold{
    constructor(){
        this.channel = 0;
        this.start = 0;
        this.length = 0;
    }
}
function readchart() {
    fetch(charturl).
        then(response => {
            if (response.ok){
                return response.text();
            }
            else {
                throw new Error("Map not found error! status: "+ response.status);
            }
        }).
        then(text => {
            let chartdata = text;
            chartinfo = parsechart(chartdata);
        });
        return;
}

function parsechart(chartdata){
    let chartinfo, splitted, header, expansion, main, maxbeat;
    // separate the array 
    // when found *----------------------
    splitted = chartdata.split("*----------------------");
    header = splitted[1];
    if (chartdata.match(/EXPANSION FIELD/)){
        expansion = splitted[2];
        main = splitted[3];
    }
    else{
        main = splitted[2];
    }

    // get the information of the Chart
    chartinfo = new Chart();
    chartinfo.genre = header.match(/\#GENRE .+/)[0].slice(7);
    chartinfo.artist = header.match(/\#ARTIST .+/)[0].slice(8);
    chartinfo.title = header.match(/\#TITLE .+/)[0].slice(7);
    chartinfo.bpm = Number(header.match(/\#BPM \d+/)[0].slice(5));
    chartinfo.playlevel = Number(header.match(/\#PLAYLEVEL \d+/)[0].slice(11));
    chartinfo.rank = Number(header.match(/\#RANK \d+/)[0].slice(6));
    try{
        chartinfo.subtitle = header.match(/\#SUBTITLE .+/)[0].slice(10);
    }
    catch{
        chartinfo.subtitle = "";
    }
    let bpms = {};
    // console.log(bpms)
    // match the #BPMxx N
    matchbpm = header.match(/\#BPM\d+ \d+(\.\d+)?/g);
    if (matchbpm != null){
        for (let i of matchbpm){
            bpms[i.slice(4,6)] = Number(i.slice(7));
        }
    }
    let stops = {};
    // match the #STOPxx N
    matchstop = header.match(/\#STOP\d+ \d+/g);
    if (matchstop != null){
        for (let i of matchstop){
            stops[i.slice(5,7)] = Number(i.slice(8));
        }
    }
    // audios = {};
    // // get the audio
    // for (let i of header.split("\r\n")){
    //     if (i.startsWith("#WAV")){
    //         audios[i.slice(4,6)] = new Audio(dir + i.slice(7, -3) + "ogg");
    //     }
    // }
    // get the number of beats
    for (let i of main.split("\r\n").reverse()){
        if (i.startsWith("#")){
            maxbeat = Number(i.slice(1,4));
            chartinfo.maxbeat = maxbeat;
            break;
        }
    }
    bars = [];
    // in every bar[i] there is an an order list of the groups of notes
    // with the respective channels to play them and time to play the next group

    let barelements = []; // auxiliar array to store the elements of the bar
    let bpm = chartinfo.bpm;
    let holdstart = [null, null, null, null, null, null, null];
    for (let i = 0; i <= maxbeat; i++){
        bars[i] = new Bar();
        barelements[i] = [];
    }
    for (let i of main.split("\r\n")){
        if (!i.startsWith("#")){
            continue;
        }
        barnumber = Number(i.slice(1,4));
        channel = i.slice(4,6);
        // if beat 02 beat is not 4/4
        // put in front the change of time signature
        if (i.slice(4,6) == "02"){
            bars[barnumber].sigchange = true;
            bars[barnumber].sigvalue = Number(i.slice(7));
            continue;
        }
        // bpm change
        if (i.slice(4,6) == "03"){
            bars[barnumber].bpmchange = true;
            objects = i.slice(7).match(/.{1,2}/g).map(x => parseInt(x, 16));
            // the index of first non null element
            let index = objects.findIndex(x => x != 0);
            bars[barnumber].bpmvalue = Math.max(...objects);
            bars[barnumber].sigvalue *= ((objects.length - index) + index * bars[barnumber].bpmvalue / bpm) / objects.length;
            bpm = bars[barnumber].bpmvalue;
            continue;
        }
        // bpm change 2
        if (i.slice(4,6) == "08"){
            bars[barnumber].bpmchange = true;
            bars[barnumber].bpmvalue = bpms[i.slice(7, 9)];
            bpm = bars[barnumber].bpmvalue;
        }
        // stop
        if (i.slice(4,6) == "09"){
            bars[barnumber].stop = true;
            bars[barnumber].stopvalue = stops[i.slice(7, 9)];
        }
        bars[barnumber].bpmvalue = bpm;
        // if it's click notes
        if (channel[0] == "1" || channel[0] == "5"){
            objects = i.slice(7).match(/.{1,2}/g);
            for (let j = 0; j < objects.length; j++){
                // if it's null
                if (objects[j] == "00"){
                    continue;
                }
                if (channel[1] > "7"){
                    channel = channel[0] + (channel[1] - 2);
                    // console.log(channel)
                }
                let hold = null;
                // if it's hold notes
                if (channel[0] == "5"){
                    // if it's the start of the hold note
                    if (holdstart[channel[1]] == null){
                        holdstart[channel[1]] = Number(barnumber) + j / objects.length;
                    }
                    // if it's the end of the hold note
                    else{
                        hold = new Hold();
                        hold.channel = channel;
                        hold.start = holdstart[channel[1]] % 1;
                        hold.length = 0;
                        // calculate the length of the hold note
                        // console.log(bars[Math.floor(holdstart[channel[1]])], holdstart[channel[1]]);
                        // console.log(holdstart[channel[1]]);
                        let tmpbpm = bars[Math.floor(holdstart[channel[1]])].bpmvalue;
                        // console.log(barnumber, bpm, tmpbpm, hold.length, bars[barnumber].bpmvalue, objects.length, holdstart[channel[1]]);
                        startlen = bars[Math.floor(holdstart[channel[1]])].sigvalue * hold.start * chartinfo.bpm / bars[[Math.floor(holdstart[channel[1]])]].bpmvalue;
                        for (let bar of bars.slice(holdstart[channel[1]], barnumber)){
                            if (bar.bpmchange == true){
                                tmpbpm = bar.bpmvalue;
                            }
                            hold.length += bar.sigvalue * chartinfo.bpm / tmpbpm;
                        }
                        hold.length += j / objects.length * bars[barnumber].sigvalue * chartinfo.bpm / bars[barnumber].bpmvalue - startlen;
                        // console.log(barnumber, bpm, tmpbpm, hold.length, bars[barnumber].bpmvalue, objects.length)
                        if (hold.length > maxlen){
                            hold.length = maxlen;
                        }
                        // console.log(hold, holdstart[channel[1]])
                        barelements[holdstart[channel[1]] - hold.start].push({
                            channel: channel,
                            object: objects[j],
                            hold: hold,
                            time: hold.start
                        });
                        holdstart[channel[1]] = null;     
                    }
                    continue;
                }
                barelements[barnumber].push({
                    channel: channel,
                    object: objects[j],
                    hold: hold,
                    time: j / objects.length
                });
                
            }
        }
        
    }
    // sort the elements of the bar by time
    for (let i of barelements){
        i.sort((a, b) => a.time - b.time);
    }
    // group the elements of the bar by time
    for (let i = 0; i < barelements.length; i++){
        let j = 0;
        while (j < barelements[i].length){
            let k = j;
            let time = barelements[i][j].time;
            let group = {
                time : time,
                channels: new Set(),
                holds: new Set()
            };
            while (k < barelements[i].length && barelements[i][k].time == time){
                if (barelements[i][k].channel[0] == "1"){
                    group.channels.add(barelements[i][k].channel);
                }
                else{
                    // console.log(barelements[i][k]);
                    group.holds.add(barelements[i][k].hold);
                }
                k++;
            }
            bars[i].notes.push(group);
            j = k;
        }
    }
    // count the total number of notes
    for (let i of bars){
        for (let j of i.notes){
            chartinfo.notecount += j.channels.size;
            chartinfo.notecount += j.holds.size * 2;
        }
    }

    return chartinfo;
}