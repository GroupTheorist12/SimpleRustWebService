
use actix_web::{get, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
extern crate serde;
use actix_cors::Cors;

use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    make: String,
    model: String,
    first_year: i32,
    last_year: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarList {
    pub cars: Vec<Car>,
}

#[derive(Serialize, Deserialize)]
pub struct CarQuery {
    pub car: String,
    pub first_year:i32,
    pub query:String
    
}

#[get("/cars/{car}/{first_year}/{query}")]
async fn get_cars(obj: web::Path<CarQuery>) -> Result<HttpResponse> {
    
    let mut file = File::open("cars.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
 
    let cl: CarList = serde_json::from_str(&buff).unwrap();


    let cq = CarQuery {
        car:obj.car.clone(),
        first_year:obj.first_year,
        query:obj.query.clone()
    };
    let vec_cars  = get_query(cl.cars, cq);
    let car_list = CarList { cars: vec_cars };

    Ok(HttpResponse::Ok().content_type("application/json").json(car_list)
    )
    

}

pub fn get_query(vcars: Vec<Car>, q: CarQuery) -> Vec<Car> {
    let mut v = match q.query.as_str().trim() {
  
        "make:eq" =>
        {
          vcars.into_iter().filter(|s| s.make == q.car).collect()
  
        }
        "make:like" =>
        {
          vcars.into_iter().filter(|s| s.make.as_str().find(q.car.as_str()) >= Some(0)).collect()
  
        }
        "make:eq,first_year:gt" =>
        {
          vcars.into_iter().filter(|s| s.make == q.car && s.first_year > q.first_year).collect()
  
        }
        "make:eq,first_year:gte" =>
        {
          vcars.into_iter().filter(|s| s.make == q.car && s.first_year >= q.first_year).collect()
  
        }
        "make:eq,first_year:eq" =>
        {
          vcars.into_iter().filter(|s| s.make == q.car && s.first_year == q.first_year).collect()
  
        }
        "make:eq,first_year:lt" =>
        {
          vcars.into_iter().filter(|s| s.make == q.car && s.first_year < q.first_year).collect()
  
        }
        "make:eq,first_year:lte" =>
        {
          
          vcars.into_iter().filter(|s| s.make == q.car && s.first_year <= q.first_year).collect()
  
        }
        "make:like,first_year:gt" =>
        {
          vcars.into_iter().filter(|s| s.make.as_str().find(q.car.as_str()) >= Some(0) && s.first_year > q.first_year).collect()
  
        }
        "make:like,first_year:gte" =>
        {
          vcars.into_iter().filter(|s| s.make.as_str().find(q.car.as_str()) >= Some(0) && s.first_year >= q.first_year).collect()
  
        }
        "make like and first_year <" =>
        {
          vcars.into_iter().filter(|s| s.make.as_str().find(q.car.as_str()) >= Some(0) && s.first_year < q.first_year).collect()
  
        }
        "make:like,first_year:lte" =>
        {
          vcars.into_iter().filter(|s| s.make.as_str().find(q.car.as_str()) >= Some(0) && s.first_year <= q.first_year).collect()
  
        }
        &_ =>
        {
          let mut vv :Vec<Car> = Vec::new();
  
          for i in 0..20 {
            let c = Car {
              make: vcars[i].make.clone(),
              model: vcars[i].model.clone(),
              first_year: vcars[i].first_year,
              last_year: vcars[i].last_year 
  
            };
            vv.push(c)
          }
          vv
        }
  
  
    };
  
    //v
    v.sort_by(|a, b| a.make.cmp(&b.make));
    v

  
  }
  
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new()
    .service(get_cars)
    .wrap(Cors::permissive()))
        .bind("127.0.0.1:9100")?
        .run()
        .await
}


