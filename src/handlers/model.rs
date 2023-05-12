
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Duration, Utc, offset, Local};




pub fn initiate_database() -> Result<()> {
    let conn = Connection::open("parkinglot.db")?;

    println!("Initiating DB...");

    conn.execute(
        "create table if not exists block (
             id integer primary key,
             name text not null unique,
             availability int not null,
             hourly_rate int not null,
             floor int not null,
             vehicle_type TEXT not null
         )",
        (),
    )?;
    conn.execute(
        "create table if not exists parking (
             id integer primary key,
             vehicle_id text not null,
             checkin TEXT not null,
             checkout TEXT,
             block_id integer not null references block(id)
         )",
        (),
    )?;
    Ok(())
}


#[derive(Serialize, Deserialize)]
pub struct RequestBlock {
    name: String,
    availability: i32,
    hourly_rate: i64,
    floor: i8,
    vehicle_type: String
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    id: i64,
    name: String,
    availability: i32,
    hourly_rate: i64,
    floor: i8,
    vehicle_type: String
}

pub fn create_block(request: RequestBlock) -> Result<Block> {
    let conn = Connection::open("parkinglot.db")?;

    conn.execute(
        "INSERT INTO block (name, availability, hourly_rate, floor, vehicle_type) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&request.name, &request.availability, &request.hourly_rate, &request.floor, &request.vehicle_type),
    )?;

    let id: i64 = conn.last_insert_rowid();
    let resp: Block = Block{
        id: id,
        name: request.name,
        availability: request.availability,
        hourly_rate: request.hourly_rate,
        floor: request.floor,
        vehicle_type: request.vehicle_type
    };
    Ok(resp)
}


#[derive(Serialize, Deserialize)]
pub struct AvailabilitiesVehicleType {
    name: String,
    availability: i32,
    hourly_rate: i64,
    floor: i8
}


pub fn get_block_availability_by_vehicle_type(vehicle_type_input: &str) -> Result<Vec<AvailabilitiesVehicleType>> {
    let conn = Connection::open("parkinglot.db")?;

    let mut availabilities = vec![];

    let mut stmt = conn.prepare("SELECT name, availability, hourly_rate, floor FROM block WHERE vehicle_type = :vehicle_type;")?;

    let availability_iter = stmt.query_map(&[(":vehicle_type", vehicle_type_input)], |row| {
        Ok(AvailabilitiesVehicleType {
            name: row.get(0)?,
            availability: row.get(1)?,
            hourly_rate: row.get(2)?,
            floor: row.get(3)?
        })
    })?;

    for ava in availability_iter {
        availabilities.push(ava.unwrap());
    }

    Ok(availabilities)
}

pub fn get_availability_by_block_id(id: i64) -> Result<i64> {
    let conn = Connection::open("parkinglot.db")?;

    let i: i64 = conn.query_row(
        "SELECT availability FROM block WHERE id = ?1",
        &[&id],
        |r| r.get(0)
    ).expect("select failed");

    Ok(i)
}

// pub fn get_block_by_id(id: i64) {
//     let conn = Connection::open("parkinglot.db")?;

// }

#[derive(Serialize, Deserialize)]
pub struct ParkingEntry {
    pub vehicle_id: String,
    pub block_id: i64
}


pub fn get_total_vehicle_in_block(block_id: i64) -> Result<i64> {
    let conn = Connection::open("parkinglot.db")?;

    let i: i64 = conn.query_row(
        "SELECT count(*) FROM parking WHERE block_id = ?1 AND checkout is NULL",
        &[&block_id],
        |r| r.get(0)
    ).expect("select failed");

    Ok(i)
}


#[derive(Serialize, Deserialize)]
pub struct ParkingReq {
    pub vehicle_id: String,
    pub block_id: i64
}

#[derive(Serialize, Deserialize)]
pub struct ParkingResp {
    id: i64,
    vehicle_id: String,
    checkin: String,
    block_id: i64
}

pub fn reserve_block(vehicle_id: String, block_id: i64, checkin_dt: DateTime<Local>) -> Result<ParkingResp> {
    let conn = Connection::open("parkinglot.db")?;

    conn.execute(
        "INSERT INTO parking (vehicle_id, checkin, block_id) VALUES (?1, ?2, ?3)",
        (&vehicle_id, &checkin_dt.to_rfc3339(), &block_id),
    )?;

    let id: i64 = conn.last_insert_rowid();

    let parking_resp: ParkingResp = ParkingResp {
        id: id,
        vehicle_id: vehicle_id,
        checkin: checkin_dt.to_rfc3339(),
        block_id: block_id 
    }; 

    Ok(parking_resp)

} 


#[derive(Serialize, Deserialize)]
pub struct Invoice {
    pub duration_hour: i64, 
    pub amount: i64
}


#[derive(Serialize, Deserialize)]
pub struct Checkout {
    pub vehicle_id: String,
    pub checkin: String,
    pub checkout: String,
    pub block_id: i64
}


#[derive(Serialize, Deserialize)]
pub struct CheckoutInvoice {
    pub vehicle_id: String,
    pub checkin: String,
    pub checkout: String,
    pub block_id: i64,
    pub invoice: Invoice
}


pub fn get_hourly_rate_by_block_id(block_id: i64) -> Result<i64> {
    let conn = Connection::open("parkinglot.db")?;

    let i: i64 = conn.query_row(
        "SELECT hourly_rate FROM block WHERE id = ?1",
        &[&block_id],
        |r| r.get(0)
    ).expect("select failed");

    Ok(i)
}


pub fn checkout_parking(vehicle_id_input: &str, checkout_dt: DateTime<Local>) -> Result<Checkout> {
    let conn = Connection::open("parkinglot.db")?;

    conn.execute(
        "UPDATE parking SET checkout = ?1 WHERE vehicle_id = ?2",
        (&checkout_dt.to_rfc3339(), vehicle_id_input),
    )?;

    let mut stmt = conn.prepare("SELECT vehicle_id, checkin, checkout, block_id FROM parking WHERE vehicle_id = :vehicle_id;")?;

    let checkout_iter = stmt.query_map(&[(":vehicle_id", vehicle_id_input)], |row| {
        Ok(Checkout {
            vehicle_id: row.get(0)?,
            checkin: row.get(1)?,
            checkout: row.get(2)?,
            block_id: row.get(3)?
        })
    })?;

    for ci in checkout_iter {
        return Ok(ci.unwrap());
    }

   panic!("Cannot fing the parking data");

} 
