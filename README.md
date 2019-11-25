#Key-Value Service
rust-key-value is a key value Pair service, which is developed in rust.

#Installation
Use cargo install 'name of the crate to be installed'

#Dependencies
Diesel - Diesel is a query builder designed to interact with sql.
Actix-web - Actix-web is a web framework for Rust. Actix-web will expose an HTTP server contained within a native executable.

#Build and Run
Use "cargo run" to run the app. Cargo Run will start the server on default port 8080.

#Database
Used mysql as database 

#Apis
CRUD apis are developed through which a key value is added, modified and deleted

1.Add a record(Key-Value pair) in db
curl -X POST \
  http://localhost:8080/ \
  -H 'Content-Type: application/json' \
  -H 'Postman-Token: b7ff66d0-649e-4b90-84e9-82141d4c4597' \
  -H 'cache-control: no-cache' \
  -d '{
    "name": "abcder",
    "value": "1"
}'

above request will add a key('abcder') whose value is '1'

2.Update a existing record
curl -X PUT \
  http://localhost:8080/ \
  -H 'Accept: */*' \
  -H 'Accept-Encoding: gzip, deflate' \
  -H 'Cache-Control: no-cache' \
  -H 'Connection: keep-alive' \
  -H 'Content-Length: 35' \
  -H 'Content-Type: application/json' \
  -H 'Host: localhost:8080' \
  -H 'Postman-Token: 59b5f8d9-b214-43f7-8bd0-1e398bed34ca,66690e1e-6182-46b9-ac1d-654fcbac56d8' \
  -H 'User-Agent: PostmanRuntime/7.19.0' \
  -H 'cache-control: no-cache' \
  -d '{
    "name": "1234",
    "value": "23"
}'

Above curl will update the existing key with new value

3.Get a record or records of specific keys
curl -X GET \
  'http://localhost:8080/?keys=abcd,1234' \
  -H 'Content-Type: application/json' \
  -H 'Postman-Token: 4a8ddbb6-54d6-48a9-815d-1e5be9ff220a' \
  -H 'cache-control: no-cache' 
  
 Pass keys as comma separated 
 
4.Fetch all records from db
curl -X GET \
  http://localhost:8080/all \
  -H 'Content-Type: application/json' \
  -H 'Postman-Token: ed064446-98d1-4b58-abe3-75254c17509e' \
  -H 'cache-control: no-cache'
  
5.delete a particular record from db
curl -X DELETE \
    http://localhost:8080/ \
    -H 'Accept: */*' \
    -H 'Accept-Encoding: gzip, deflate' \
    -H 'Cache-Control: no-cache' \
    -H 'Connection: keep-alive' \
    -H 'Content-Length: 19' \
    -H 'Content-Type: application/json' \
    -H 'Host: localhost:8080' \
    -H 'Postman-Token: 23766ad0-0bb6-406e-883c-f3ce5b7e785f,94da272e-4c44-4067-b3be-1c28ae6f3d02' \
    -H 'User-Agent: PostmanRuntime/7.19.0' \
    -H 'cache-control: no-cache' \
    -d '{
      "name": "abcd"
  }'
  





