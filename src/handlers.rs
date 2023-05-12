use serde::{Deserialize, Serialize};
use std::io;
use crate::handlers::model::{initiate_database, create_block, Block, RequestBlock, get_block_availability_by_vehicle_type, AvailabilitiesVehicleType,
    get_availability_by_block_id, ParkingEntry, get_total_vehicle_in_block, reserve_block, checkout_parking, get_hourly_rate_by_block_id, Invoice, CheckoutInvoice};
use chrono::{DateTime, Duration, Utc, offset, Local};


pub mod model;


pub fn entry_point() -> String {
    return String::from("Hello All!");
}


pub fn initiate_database_handler() -> String {

    let ib = initiate_database();
    let message = match ib {
        Ok(_) => String::from("Successfully initiate database"),
        Err(error) => String::from(&format!("Error {}", error))
    };

    return message;
}


pub fn create_block_handler(req_data: &str) -> Result<String, io::Error> {

    let block: RequestBlock = serde_json::from_str(req_data)?;

    let resp = create_block(block);
    
    let resp = match resp {
        Ok(resp) => resp,
        Err(error) => panic!()
    };

    let resp: String = serde_json::to_string(&resp)?;

    Ok(resp)
}

pub fn get_availabilities_by_vehicle_type(vechicle_type: &str) -> Result<String, io::Error> {

    let availabilities = get_block_availability_by_vehicle_type(vechicle_type);

    let resp = match availabilities {
        Ok(resp) => resp,
        Err(error) => panic!()
    };

    let mut new_resp = vec![];
    for mut r in resp.into_iter() {
        let resp_tot_reserved = get_total_vehicle_in_block(r.id);

        let tot_reserved: i64 = match resp_tot_reserved {
            Ok(a) => a,
            Err(_) => 0
        };

        r.availability = r.availability - tot_reserved;
        new_resp.push(r);
    }

    let new_resp: String = serde_json::to_string(&new_resp)?;

    Ok(new_resp)
}



pub fn do_parking_handler(req_data: &str) -> Result<String, io::Error>  {
    let entry: ParkingEntry = serde_json::from_str(req_data)?;

    let resp_avail = get_availability_by_block_id(entry.block_id);

    let avail: i64 = match resp_avail {
        Ok(a) => a,
        Err(_) => 0
    };

    let resp_tot_reserved = get_total_vehicle_in_block(entry.block_id);

    let tot_reserved: i64 = match resp_tot_reserved {
        Ok(a) => a,
        Err(_) => 0
    };

    if tot_reserved < avail {
        let checkin_dt: DateTime<Local> = offset::Local::now();
        let resp = reserve_block(entry.vehicle_id, entry.block_id, checkin_dt);
        
        let resp = match resp {
            Ok(resp) => resp,
            Err(error) => panic!()
        };

        let resp: String = serde_json::to_string(&resp)?;

        return Ok(resp);
    } else {
        panic!("Full reserved");
    }
}


pub fn checkout_handler(vehicle_id: &String) -> Result<String, io::Error> {
    let checkout_dt: DateTime<Local> = offset::Local::now();

    let parking = checkout_parking(vehicle_id, checkout_dt);

    let resp = match parking {
        Ok(resp) => resp,
        Err(error) => panic!("Error retrieving parking {}", error)
    };

    let hourly_rate_resp = get_hourly_rate_by_block_id(resp.block_id);
    let hourly_rate: i64 = match hourly_rate_resp {
        Ok(resp) => resp,
        Err(error) => panic!("Error retrieving horly rate {}", error)
    };

    let checkin_dt = DateTime::parse_from_rfc3339(&resp.checkin);
    let checkin_dt = match checkin_dt {
        Ok(resp) => resp,
        Err(error) => panic!("Cannot parse the checkin {}", error)
    };
    let diff = checkout_dt.signed_duration_since(checkin_dt);
    let hours = diff.num_hours();

    let invoice: Invoice = Invoice {
        duration_hour: hours + 1,
        amount: (hours + 1) * hourly_rate
    };

    let resp_checkout_invoice: CheckoutInvoice = CheckoutInvoice {
        vehicle_id: resp.vehicle_id,
        checkin: resp.checkin,
        checkout: resp.checkout,
        block_id: resp.block_id,
        invoice: invoice
    };

    let resp_checkout_invoice: String = serde_json::to_string(&resp_checkout_invoice)?;

    Ok(resp_checkout_invoice)

}
