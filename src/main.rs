#![windows_subsystem = "windows"]

use gtk::{glib, gio, prelude::*};
use std::{fs::File, io::{prelude::*, BufReader}};
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]

struct Record {
    llegada: String,
    salida: String,
    apartamento: String,
    huésped: String,
    portal: String,
    teléfono: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    adultos: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    niños: Option<u32>,

}

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.builder_basics")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let ui_src = include_str!("./resources/ui/smoobu-mami.ui");
    let builder = gtk::Builder::from_string(ui_src);

    let window1 = builder
        .object::<gtk::ApplicationWindow>("window1")
        .expect("Couldn't get window");
    window1.set_application(Some(application));
    let button1 = builder
        .object::<gtk::Button>("button1")
        .expect("Couldn't get button");
    let filedialog1 = builder
        .object::<gtk::FileDialog>("filedialog1")
        .expect("Couldn't get filedialog");
    let text_view = builder
        .object::<gtk::TextView>("textview1")
        .expect("Couldn't get text_view");
    let dropdown1 = builder
        .object::<gtk::DropDown>("dropdown1")
        .expect("Couldn't get dropdown1");
    let about1 = builder
        .object::<gtk::AboutDialog>("about1")
        .expect("Couldn't get about1");

    
    let dropdown2 = dropdown1.clone();
    dropdown1.connect_selected_item_notify(glib::clone!(@weak window1, @weak text_view => move |_| {
        let dropdown3 = dropdown2.clone();
        filedialog1.open(Some(&window1), gio::Cancellable::NONE, move |file| {
            if let Ok(file) = file {
                let filename = file.path().expect("Couldn't get file path");
                let file = File::open(filename).expect("Couldn't open file");
                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);
                let selected = dropdown3.selected();

                let deser_string = deserialize(&mut contents, selected);
                text_view.buffer().set_text(&deser_string);
            }
        });
    })
    );
    button1.connect_clicked(glib::clone!(@weak window1, @weak text_view => move |_| {
        about1.present();
    }));


    window1.present();
}

fn deserialize(contents: &mut String, selected: u32) -> String {
    let mut contents_comma = contents.trim().replace(';', ",");
    contents_comma = contents_comma.replace("Portal de reserva", "Portal");
    contents_comma.push_str(",");

    let mut rdr = ReaderBuilder::new()
        //.delimiter(b';')
        .flexible(true)
        //.quoting(false)
        .from_reader(contents_comma.as_bytes());
    
    let mut return_string: String = String::new();
    let mut reservas: Vec<Record> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result.unwrap_or_else(|error| panic!("{error}"));
        reservas.push(record);
    }

    reservas.sort_by(|a, b| fecha2int(&b.llegada).cmp(&fecha2int(&a.llegada)));

    for i in [reservas.len(), reservas.len()-1] {
        if reservas[i-1].llegada[3..5].parse::<i32>().unwrap() != reservas[i-1].salida[3..5].parse::<i32>().unwrap() {
            reservas.swap_remove(i-1);
        }
    }

    reservas.reverse();


    for reser in reservas.iter_mut() {
        doomsday(&mut reser.llegada);
        doomsday(&mut reser.salida);
        match selected {
            0 => {
                if reser.apartamento == "Blu Vegueta".to_string() {
                    return_string.push_str(&format!("{} hasta {}\n{}, nombre: {}, huéspedes: {}\n{} || {}\n\n", reser.llegada, reser.salida, reser.apartamento, reser.huésped, personas(&reser.adultos, &reser.niños), reser.teléfono, reser.portal));
                }
            }
            1 => {
                if reser.apartamento == "Apartamentos Vegueta".to_string() {
                    return_string.push_str(&format!("{} hasta {}\n{}, nombre: {}, huéspedes: {}\n{} || {}\n\n", reser.llegada, reser.salida, reser.apartamento, reser.huésped, personas(&reser.adultos, &reser.niños), reser.teléfono, reser.portal));
                }
            }
            2 => {
                return_string.push_str(&format!("{} hasta {}\n{}, nombre: {}, huéspedes: {}\n{} || {}\n\n", reser.llegada, reser.salida, reser.apartamento, reser.huésped, personas(&reser.adultos, &reser.niños), reser.teléfono, reser.portal));
            }
            _ => panic!(),
        }
    }
    return return_string;
}
fn personas(adultos: &Option<u32>, niños: &Option<u32>) -> u32 {
    let mut personas = 0;
    match adultos {
        Some(p) => personas = personas + p,
        None => personas = personas,
    }
    match niños {
        Some(p) => personas = personas + p,
        None => personas = personas,
    }

    return personas;
}

fn fecha2int(fechastr: &String) -> u32 {
    let dia = fechastr[0..2].parse::<i32>().unwrap();
    let mes = fechastr[3..5].parse::<i32>().unwrap();
    let anno = fechastr[6..8].parse::<i32>().unwrap();

    let tmes;
    match mes {
        1 => tmes = 0,
        2 => tmes = 31,
        3 => tmes = 59,
        4 => tmes = 90,
        5 => tmes = 120,
        6 => tmes = 151,
        7 => tmes = 181,
        8 => tmes = 212,
        9 => tmes = 243,
        10 => tmes = 273,
        11 => tmes = 304,
        12 => tmes = 334,
        _ => panic!(),
    }

    let int: u32 = (anno*365 + (anno/4) - (anno/100) + (anno/400) + tmes + dia).try_into().unwrap();
    int
}

fn doomsday(fechastr: &mut String) {
    let dia = fechastr[0..2].parse::<i32>().unwrap();
    let mes = fechastr[3..5].parse::<i32>().unwrap();
    let anno = fechastr[6..8].parse::<i32>().unwrap();

    let adia = ((2 + (5*(anno % 4)) + (4*(anno % 100)) + (6*(anno % 400))) % 7) + 7;
    let mut mdia = 0;
    let nmes: String;
    let bisiesto = anno % 4 == 0 && (anno % 100 != 0 || anno % 400 == 0);
    
    match mes {
        1 => {
            if bisiesto == true {
                mdia = (adia - 4) % 7;
            }
            if bisiesto == false {
                mdia = (adia - 3) % 7;
            }
            nmes = "enero".to_string();
        }
        2 => {
            if bisiesto == true {
                mdia = (adia - 1) % 7;
            }
            if bisiesto == false {
                mdia = (adia - 7) % 7;
            }
            nmes = "febrero".to_string();
        }
        3 => {
            mdia = (adia - 7) % 7;
            nmes = "marzo".to_string();
        }
        4=> {
            mdia = (adia - 4) % 7;
            nmes = "abril".to_string();
        }
        5 => {
            mdia = (adia - 2) % 7;
            nmes = "mayo".to_string();
        }
        6 => {
            mdia = (adia - 6) % 7;
            nmes = "junio".to_string();
        }
        7 => {
            mdia = (adia - 4) % 7;
            nmes = "julio".to_string();
        }
        8 => {
            mdia = (adia - 1) % 7;
            nmes = "agosto".to_string();
        }
        9 => {
            mdia = (adia - 5) % 7;
            nmes = "septiembre".to_string();
        }
        10 => {
            mdia = (adia - 3) % 7;
            nmes = "octubre".to_string();
        }
        11 => {
            mdia = (adia - 7) % 7;
            nmes = "noviembre".to_string();
        }
        12 => {    
            mdia = (adia - 5) % 7;
            nmes = "diciembre".to_string();
        }
        _ => panic!(),
        
    }
    let ddia = (mdia + dia) % 7;
    let dsemana: String;

    match ddia {
        0 => dsemana = "Domingo".to_string(),
        1 => dsemana = "Lunes".to_string(),
        2 => dsemana = "Martes".to_string(),
        3 => dsemana = "Miércoles".to_string(),
        4 => dsemana = "Jueves".to_string(),
        5 => dsemana = "Viernes".to_string(),
        6 => dsemana = "Sábado   ".to_string(),
        _ => panic!(),
    }

    *fechastr = format!("{dsemana} {dia} de {nmes} de 20{anno}");

}
