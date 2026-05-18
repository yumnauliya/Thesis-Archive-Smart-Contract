# Thesis Archive Smart Contract

A blockchain-based thesis archive system built using Soroban Smart Contracts on the Stellar network.

This project was created to help Computer Engineering students manage and search archived thesis data from alumni based on specialization categories such as:

- Network Engineering
- Embedded Systems
- Multimedia
- Software Engineering

The system stores thesis metadata on blockchain storage to ensure data integrity, transparency, and immutability.

---

# Features

- Add new thesis data
- View all thesis archives
- Delete thesis data by ID
- Filter thesis by year
- Filter thesis by category
- Blockchain-based storage
- Immutable archive records

---

# Thesis Data Structure

Each thesis contains:

| Field | Description |
|---|---|
| id | Unique thesis ID |
| title | Thesis title |
| author | Author/alumni name |
| year | Thesis publication year |
| category | Thesis specialization category |
| abstract_text | Thesis abstract |

---

# Technologies Used

- Rust
- Soroban 
- Stellar Soroban Smart Contract
- WASM (WebAssembly)

---

# Project Structure

```bash
.
├── src
│   ├── lib.rs
│   └── test.rs
├── Cargo.toml
└── README.md