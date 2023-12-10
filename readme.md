# BMS file parser

## Descrizione

I file BMS [(wikipedia)](https://en.wikipedia.org/wiki/Be-Music_Source) sono il formato standard per la creazione di mappe per giochi ritmici della tipologia beatmania.
Il programma è stato sviluppato in rust per leggere i file BMS e per estrarre le informazioni contenute al loro interno. In particolare il file viene convertito in file json che può essere letto e giocato su [suqichen.ddns.net](https://suqichen.ddns.net).

## Compilazione ed esecuzione

- `rustc src/main.rs`
- `./main <file.bms> (<output_filename.json>)`

## Formato file json

```json
{
"chartinfo": {
    # informazioni sulla mappa
    "genre": #"Genere",
    "title": #"Titolo",
    "artist": #"Artista",
    "bpm": #bpm,
    "playlevel": difficoltà,
    "rank": numero di scelta,
    "subtitle": secondo titolo,
},
"bars": [
    #lista di bar
    {
        "notes": #Lista di gruppi di note
            [{
                "time": #tempo di spawn,
                "channels": [ #lista di canali in cui spawnare la nota click],
                "holds": [#lista di note da tenere premuto]
            }],
        "sigchange": #bool segnale di cambio di signature,
        "sigvalue": #nuovo valore di signature,
        "bpmchange": #bool segnale di cambio di bpm,
        "bpmvalue": #nuovo valore di bpm,
        "stop": #bool segnale di stop,
        "stopvalue": #tempo di stop,
    }
    ...
]
}
```
## Esempio
- `rustc src/main.rs`
- `./main examples/Altros_a.bms` 
- copia il contuto del file `examples/Altros_a.json`
- vai su [suqichen.ddns.net](https://suqichen.ddns.net)
- F12 -> Console
- avvia una difficoltà qualsiasi di Altros
- sulla console scrivi:
- `l = <incolla il contenuto del file json>`
- `chartinfo = l.chartinfo`
- `bars = l.bars`
- clicca qualsiasi tasto per far partire la mappa

# Differenze con vecchia implementazione e Rust
La precedente implementazione che si trova su original/parsechart.js è stata scritta in javascript.

### Formato progetto
Inanzitutto il codice è stato divisone in funzioni e in moduli. main.rs chiama le funzioni di parsechart.rs che a sua volta usa le classi di chart.rs. Invece in parsechart.js tutto il codice è stato scritto in un unico file in unica funzione.

### Classi
Nell'originale era presente solo il costruttore e non è stata necessaria l'implementazione di Note e NoteGroup dove si è semplicemente utilizzato un oggetto javascript. 
Invece in Rust sono state implementate le funzioni di utilitità display() e to_string() per la formattazione e Default() per la creazione di oggetti vuoti.
Inoltre poter usare ```Hashset<Option<Hold>>``` è stato necessario implementare PartialEq e Hash per Hold.

### Casting
In javascript non è stato necessario prestare attenzione al casting, mentre in Rust è stato necessario prestare particolare attenzione al casting in ogni singola operazione (usize, addizione moltiplicazione tra int float ecc).

### RAII e mut
In Rust è stato necessario prestare attenzione alla gestione delle variabili. Molto spesso durante l'implementazione mi sono trovato a usare per sbaglio variabili che erano già mosse di ownership (esempio creazione di un oggetto di un vettore e poi riutilizzo delle variabili che appartenevano all'oggetto).
Grazie al compilatore sono state identificate tutte le variabili che non era necessario dichiarare mutabili.
