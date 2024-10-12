# Rust Todo API

## Overview

This is a simple Todo API built using the Actix framework in Rust. It allows users to create, read, update, and delete todo items.

## API Endpoints

### 1. Create Todo

- **Endpoint:** `POST /todos`
- **Description:** Create a new todo item.
- **Request Body:**
  ```json
  {
    "title": "string",
    "completed": false
  }
  ```
- **Response:**
  - **201 Created**
  ```json
  {
    "id": 1,
    "title": "string",
    "completed": false
  }
  ```

### 2. Get All Todos

- **Endpoint:** `GET /todos`
- **Description:** Retrieve all todo items.
- **Response:**
  - **200 OK**
  ```json
  [
    {
      "id": 1,
      "title": "string",
      "completed": false
    },
    {
      "id": 2,
      "title": "another string",
      "completed": true
    }
  ]
  ```

### 3. Get Todo by ID

- **Endpoint:** `GET /todos/{id}`
- **Description:** Retrieve a specific todo item by its ID.
- **Response:**
  - **200 OK**
  ```json
  {
    "id": 1,
    "title": "string",
    "completed": false
  }
  ```
  - **404 Not Found** (if the todo does not exist)

### 4. Update Todo

- **Endpoint:** `PUT /todos/{id}`
- **Description:** Update a specific todo item.
- **Request Body:**
  ```json
  {
    "title": "updated string",
    "completed": true
  }
  ```
- **Response:**
  - **200 OK**
  ```json
  {
    "id": 1,
    "title": "updated string",
    "completed": true
  }
  ```
  - **404 Not Found** (if the todo does not exist)

### 5. Delete Todo

- **Endpoint:** `DELETE /todos/{id}`
- **Description:** Delete a specific todo item.
- **Response:**
  - **204 No Content** (if successful)
  - **404 Not Found** (if the todo does not exist)

## Running the API

To run the API, ensure you have Rust and Cargo installed. Then, clone the repository and run the following commands:

```bash
cargo run
```

The API will be available at `http://localhost:8000`.

## Testing the API

You can use tools like Postman or curl to test the API endpoints. Here are some example curl commands:

- Create a Todo:

  ```bash
  curl -X POST http://localhost:8000/todos -H "Content-Type: application/json" -d '{"title": "New Todo", "completed": false}'
  ```

- Get All Todos:

  ```bash
  curl http://localhost:8000/todos
  ```

- Update a Todo:

  ```bash
  curl -X PUT http://localhost:8000/todos/1 -H "Content-Type: application/json" -d '{"title": "Updated Todo", "completed": true}'
  ```

- Delete a Todo:
  ```bash
  curl -X DELETE http://localhost:8000/todos/1
  ```

## Conclusion

This documentation provides a basic overview of the Todo API built with Actix in Rust. For further details, please refer to the source code and comments within the project.
