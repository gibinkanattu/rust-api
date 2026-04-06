# rust-api
Rust api to fetch data from mongodb and influxdb


## Features
* Token is generated using credentials
* Token based authentication is required for each api endpoint
* API server status can be checked via ping endpoint


## API Endpoints
1. test/ping [GET]
   </br>Check the ping status of api server
2. test/token [POST]
</br> Get the token by passing the credentials. Token is valid for a day and can be reused
3. test/mongodb [GET]
</br> Get the data from mongodb database. Token needs to passed for authentication