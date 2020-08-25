# Rust CRUD app
### How to run this project? 
go to root directory and run the command below
```
cargo run
```
It will start your server in local machine with port 8088. Remember we are using aws dynamodb, make sure to run 

```
aws sts get-caller-identity
```
to verify that you have valid token and access to your account. 

### How to make call to webservice
Here is few command: 

For root directory run command below: 
```curl http://localhost:8088/```

Adding books into dynamodb
```curl -X POST http://localhost:8088/books```

helpful link: 
* https://actix.rs/docs/application/
* https://actix.rs/docs/getting-started/
