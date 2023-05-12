
# Parking Lot Using Rust

Simple parking lot system using rust REST Api and sqlite

This system consists of 3 components


* **/src/main.rs**: Rest API interface Implemented using Actix framework that containes the several endpoints

* **/src/handlers/model.rs**: Related to database operational and data structure definition

* **/src/handlers.r**: Controller, operate the logic, bridging between rest services and model


## Series of The Event How The System Works 

### 1. Admin Create the database and table

this shouldn't be like this, but for now, I think it's fine to initialize the database using this:

```
HTTP GET http://127.0.0.1:8080/initiate_db
```
Sample of successful response:
```
Successfully initiate database
```

### 2. Admin Create Block
Block is a place to put vehicles grouped by type of vehicle.
To create a block is done by this endpoint:
```
HTTP POST http://127.0.0.1:8080/block
```
Request Body (json):
Key Name | Type Data | Description 
--- | ---  | ---
name | String | Name of the block, shold be unique 
availability | Integer | Block capacity 
hourly_rate | Integer | The price to be paid per hour 
floor | Integer | the floor where the block is located
vehicle_type | String | Should be a value od enum "M", "C", or "B". "M" is for motorcycle, "C" is for car, and "B" is for bus or any other large vehicles

Sample request body:
```
{
  "name": "Block 3",
  "availability": 300,
  "hourly_rate": 1500,
  "floor": 3,
  "vehicle_type": "M"
}
```
Sample of successful response:
```
{
  "id": 3,
  "name": "Block 3",
  "availability": 300,
  "hourly_rate": 1500,
  "floor": 3,
  "vehicle_type": "M"
}
```

### 3. Check Availability
Visitors check the availability of places based on the type of vehicle using this endpoint
```
HTTP GET http://127.0.0.1:8080/block/{vehicle_type}/availabilities
```

Sample request:
```
http://127.0.0.1:8080/block/M/availabilities
```
Sample of successful response:
```
[
  {
    "id": 1,
    "name": "Block 1",
    "availability": 499,
    "hourly_rate": 1000,
    "floor": 1
  },
  {
    "id": 3,
    "name": "Block 3",
    "availability": 300,
    "hourly_rate": 1500,
    "floor": 3
  }
]
```
Visitors can select one of those block that has availability, then process to the next step.

### 4. Visitor Checkin
Visitors checkin to the parking area using this below endpoint:
```
HTTP POST http://127.0.0.1:8080/enter
```
Request Body (json):
Key Name | Type Data | Description 
--- | ---  | ---
vehicle_id | String | ID of vehicke (plate number), shold be unique 
block_id | Integer | ID of the block will be reserved

Sample request body:
```
{
    "vehicle_id": "BB I765 Ft",
    "block_id": 1
}

```
Sample of successful response:
```
{
  "id": 3,
  "vehicle_id": "BB I765 Ft",
  "checkin": "2023-05-12T21:18:04.377237168+07:00",
  "block_id": 1
}
```

### 5. Visitor Checkout
After finishing leaving the vehicle for a while, visitors can get out and do checkout using this below endpoint:
```
HTTP GET http://127.0.0.1:8080/checkout/{vehicle_id}
```

Sample request:
```
http://127.0.0.1:8080/checkout/BB%20I765%20Ft
```

Sample of successful response:
```
{
  "vehicle_id": "BB I765 Ft",
  "checkin": "2023-05-12T21:18:04.377237168+07:00",
  "checkout": "2023-05-12T21:30:12.415158427+07:00",
  "block_id": 1,
  "invoice": {
    "duration_hour": 1,
    "amount": 1000
  }
}
```
The amount is charged based on the duration of the vehicle's stay. For example:
* duration_hour < 1 hour => 1 * hourly_rate
* 1 hour < duration_hour < 2 hours => 2 * hourly rate
* etc...