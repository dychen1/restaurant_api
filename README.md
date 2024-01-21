# Restaurant API

## Description

This project is a simple REST API written in Rust using Axum and SQLx. The idea is that food items are associated with restaurant tables (and can optionally be tagged with a customer id). The API allows for create, read and delete operations the restaurant operates (update was not a part of the spec).

For the sake of simplicity, we do not introduce the concept of orders and do not need to keep track of the status of the items.

The following routes/endpoints are available:

- `/health` - Method: GET
  - Quick health check.

- `/table` - Method: PUT
  - Create a new table.

- `/table/id` - Method: GET
  - Fetch the number of seats for a table by its id.

- `/table/delete/id` - Method: DELETE
  - Delete a table by its table id. Cascades to delete all items associated with the table.

- `/items/` - Method: POST
  - Fetch a list of items for a table. Optionally, provide item and/or customer_id.

- `/items/add` - Method: PUT
  - Add a list of items to a table. The app generates a static cook time between 5-15 minutes for each item.

- `/items/delete/id` - Method: DELETE
  - Delete an item by its item id.

- `/items/delete/` - Method: DELETE
  - Delete the latest instance of an item from a table given a table id. Optionally, provide item and/or customer_id.

The data for the application is stored in a MySQL database running on a Docker container. The `mysql_db/init.sql` file contains the SQL commands to create the tables and populate some initial values.

Note: In hindsight, a simpler storage solution, like an in-memory hashmap, might have been more appropriate for the scope of this project.

## Usage

To run the project, you will need to have Rust installed (1.75.0 preferably). You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

You will also need to have Docker installed. You can install Docker by following the instructions [here](https://docs.docker.com/get-docker/).

A sample `.env` file is provided along with placeholder values for convenience. Please modify them as needed.

Once the run tools are installed and the `.env` file configured to your liking, you may run the following command to start the MySQL database and server:

```. ./run.sh```

Note: There is a quick hack in the `run.sh` script to wait for the MySQL database to be ready before starting the server. This should be replaced with a health check on the database docker container. If the application is not starting up correctly, you may increment the sleep time in the script to ensure the database container does spin up. Alternatively, you can remove the `cargo run --release` command from the runner script and start up the application manually after the database is ready.

Once the application is up and running, feel free to send requests to the API using your favorite REST client. A sample Postman collection is provided in the `postman` directory for convenience. You can import the collection into Postman and start sending requests to the API if nothing was changed in the `.env` file.

## Testing

There is a suite of integration tests that can be run using the following command:

```cargo test -- --test-threads=1```

**The `--test-threads=1` is important to ensure that the tests run in a single thread.** This is necessary because the tests are not isolated from each other and perform real database operations on the provided sample db, therefore they may interfere with each other if run asynchronously.

The idea of this suite of tests is to simulate all _standard_ "server" (app) operations that can be received from the "client" (user). There are 12 test cases in total, and they cover all the routes of the API.

`rstest` was used to parametrize test functions to cover more scenarios with fewer test functions.

Note: I omitted unit tests as there wasn't too much logic to test in the applications. Most of the operations are interactions with the database. There's some query building logic that can be checked, but would require some refactoring to make the code more testable. Along with the safety of the strict typing and compiler rules of Rust, I thought an integration test would be more useful for this case.

## Server

The server is built using the Axum framework. The server is built using the `async`/`await` syntax and is run on the Tokio runtime.

All routes are defined in `main.rs`. Utility functions such as the database connection pool and the generic response are defined in `utils` directory.

All the handlers for the routes are defined in `handlers` directory. The handler functions are pretty straight forward query builders. SQLx was interesting to use as well, challenging at first but the macros are pretty powerful as they perform compile-time checks on the queries. Pretty neat.

Models/schemas for the database and request/response contracts are defined under `models` directory. Concerning the models, I tried to reuse models as much as possible, but found it a bit challenging without the concept of inheritance in Rust. I think if I were to redo this project, I would spent more time planning out traits and identifying common methods. So lesson learned from my first Rust project. The response contracts could use some improvement for more consistency as well, but I thought it was more of a client preference and I didn't want to over-engineer it.

## Client

The client is simulated with the integration tests. The `reqwest` library is used to create a client and send HTTP requests to the server. The requests are meant to be run in a synchronous fashion as there are real database operations performed to keep the state of the database consistent (other than the auto-incrementing `id` column for `items` table) while still being able to test all the functionalities of the server.

I wasn't too sure about the spec of the client so I thought running 12 test cases in a single thread would be a good simulation. These tests could potentially be run in parallel to simulate multiple clients. The fastest way to do so would probably to just define specific tests that send read requests (GET and POST) and run them with GNU `parallel` from the command line. That will go on the Todo list.

## Todo's

- [ ] - Add a health check for the docker spec.
- [ ] - Add authentication middleware (could be as simple as API key handshake)
- [ ] - Better logging
- [ ] - Improve implementation of IntoResponse for GenericResponse
- [ ] - Dockerize application
- [ ] - Multi-threaded client simulation
