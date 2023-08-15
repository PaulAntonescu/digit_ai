use actix_web::HttpResponse;
use nalgebra::*;
use crate::model::MNIST;
use crate::model::NeuralNetwork;
use std::fs;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_files as afs;

mod model;

static FILE_PATH: &str = "/home/paul/Desktop/mnist_train.csv";
static mut MODLE: Option<NeuralNetwork> = None;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let mut model = NeuralNetwork {
        input_layer: SMatrix::<f64, 1, 784>::zeros(),
        h_weights1: SMatrix::<f64, 784, 20>::new_random().normalize(),
        h_layer:    SMatrix::<f64, 1, 20>::zeros(),
        h_weights2: SMatrix::<f64, 20, 10>::new_random().normalize(),
        output_layer: SMatrix::<f64, 1, 10>::zeros()
    };

    let mnist: Vec<MNIST> = load_minst(0.0);
    print!("Fitting data\n\0");
    model.fit(&mnist, 0, 0.8);
    /*
    print!("Second Pass\n\0");
    let mnist: Vec<MNIST> = load_minst(0.0);
    print!("Fitting data\n\0");
    model.fit(&mnist, 1, 0.4);
    */
    print!("Launching WebService\n\0");

    /*
    let test = &mnist[1001..1051];

    for data in test {
        print!("{}", data.input);
        print!("{}", model.predict(data.input));
    }
    */
    unsafe {
        MODLE = Some(model);
    }

    HttpServer::new(|| 
        App::new()
        .service(index).service(hello)
        .service(afs::Files::new("/", "../grid/dist/grid")
            .show_files_listing()
            .index_file("index.html")
            .use_last_modified(true))) 
    .bind(("localhost", 6969))?
    .run()
    .await
}

fn load_minst(threshold: f64) -> Vec<MNIST> {
    let mut mnist: Vec<MNIST> = Vec::new();
    let mut i = 0;
    for line in fs::read_to_string(FILE_PATH).unwrap().lines() {
        let line: Vec<f64> = line.split(",").map(|d: &str| d.parse::<f64>().unwrap()).collect();
        
        let mut y: Vec<f64> = Vec::new();
        line[1..].clone_into(&mut y);
        y = y.iter().map(|n| {
            if n > &230.0 { return 255.0/255.0; }
            if n > &150.0 { return 165.0/255.0; }
            if n > &100.0 { return 15.0/255.0; }
            return 0.0;
        }).collect();

        let mut x = SMatrix::<f64, 1, 10>::zeros();
        let label: usize = line[0] as usize;
        x[(0, label)] = 1.0;

        mnist.push(MNIST { input: (SMatrix::<f64, 1, 784>::from_vec(y)), expected: (x) }); //.normalize()

        //if i == 300 { break; }
        //i += 1;
        //print!("{i}\n");
    }

    mnist
}

#[get("/hello")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/ai/{matrix}")]
async fn hello(matrix: web::Path<String>) -> impl Responder {
    let input: Vec<f64> = matrix.split(",").map(|n: &str|n.parse::<f64>().unwrap()).collect();
    
    if input.len() != 784 {
        return HttpResponse::BadRequest().body("input 784 comma seperated digits");
    }
    let input: SMatrix::<f64, 1, 784> = SMatrix::<f64, 1, 784>::from_vec(input).normalize();

    unsafe {
        let m = match MODLE.as_mut() {
            Some(m) => m.predict(input),
            None => return HttpResponse::NotImplemented().body("Neural Network not Init"),
        };

        return HttpResponse::Ok().body(parse_guess(m));
    }
}

fn parse_guess(output: SMatrix::<f64, 1, 10>) -> String {
    let mut v: Vec<String> = Vec::new();
    for m in &output {
        v.push(m.to_string());
    }
    let mut result = "{ \"result\": [".to_owned();
    result.push_str(&v.join(", "));
    result.push_str("] }");
    result
}
