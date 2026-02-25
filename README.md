# FinTrack - Personal Finance Tracking API

A production-ready RESTful API for managing personal finances, built with Rust and Actix-web.

## Features

- User authentication with JWT tokens
- Category management (create, read, update, delete)
- Transaction tracking (income and expenses)
- Automatic balance calculation and validation
- Multi-user support with data isolation
- Rate limiting and CORS support
- Docker deployment ready

## Prerequisites

- Rust 1.82 or higher
- MySQL 8.0 or higher
- Docker (optional, for containerized deployment)

## Setup

1. Clone the repository

git clone https://github.com/yourusername/fintrack.git
cd fintrack

2. Create a `.env` file

DATABASE_URL=mysql://root:root@localhost:3306/fintrack
JWT_SECRET=your-secret-key-here
HOST=127.0.0.1
PORT=8080

3. Create the database

mysql -u root -p
CREATE DATABASE fintrack;

4. Run migrations

sqlx migrate run

5. Build and run

cargo build --release
cargo run

The API will be available at http://localhost:8080

## API Endpoints

### Authentication

POST /auth/sign-up - Create a new user account
POST /auth/sign-in - Sign in and receive JWT token

### User Profile (Protected)

GET /api/me - Get current user profile
POST /api/me - Update current user profile

### Categories (Protected)

GET /api/categories - List all categories
POST /api/categories - Create a new category
GET /api/categories/:id - Get a specific category
PUT /api/categories/:id - Update a category
DELETE /api/categories/:id - Delete a category
GET /api/categories/:id/transactions - Get all transactions in a category

### Transactions (Protected)

GET /api/transactions - List all transactions
POST /api/transactions - Create a new transaction
GET /api/transactions/:id - Get a specific transaction
PUT /api/transactions/:id - Update a transaction (memo/description only)
DELETE /api/transactions/:id - Delete a transaction

## Usage Examples

### Sign Up

curl -X POST http://localhost:8080/auth/sign-up \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123",
    "firstname": "John",
    "lastname": "Doe"
  }'

### Sign In

curl -X POST http://localhost:8080/auth/sign-in \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'

Response:
{
  "status": "success",
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}

### Create a Category (requires token)

curl -X POST http://localhost:8080/api/categories \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Groceries",
    "description": "Food and household items"
  }'

### Create a Transaction (requires token)

curl -X POST http://localhost:8080/api/transactions \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 1,
    "type": "DEBIT",
    "amount": 5000,
    "memo": "Weekly grocery shopping",
    "description": "Walmart - milk, bread, eggs"
  }'

Note: Amount is in cents (5000 = $50.00)

## Docker Deployment

### Build the image

docker build -t fintrack:latest .

### Run the container

docker run -d \
  --name fintrack \
  -p 8080:80 \
  -e DATABASE_URL="mysql://root:root@host.docker.internal:3306/fintrack" \
  -e JWT_SECRET="your-secret-key-here" \
  -e HOST="0.0.0.0" \
  -e PORT="80" \
  fintrack:latest

The API will be available at http://localhost:8080

## Architecture

FinTrack uses a clean layered architecture:

- API Layer: HTTP request handling and routing
- Service Layer: Business logic and validation
- Repository Layer: Database operations
- Domain Layer: Core entities and error types

This separation ensures maintainability, testability, and clear separation of concerns.

## Security Features

- JWT authentication with 4-hour token expiration
- Password hashing with bcrypt
- Ownership verification (users can only access their own data)
- Rate limiting (50 requests per 60 seconds)
- Input validation
- CORS support for frontend integration

## Transaction Types

- CREDIT: Income transactions (salary, gifts, refunds) - increases balance
- DEBIT: Expense transactions (groceries, rent, bills) - decreases balance

Transactions automatically update both user and category balances.


