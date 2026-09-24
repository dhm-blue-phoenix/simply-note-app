mod db;
mod handler;
mod models;
mod router;
mod service;

pub use router::router as notes_router;

#[cfg(test)]
mod tests;
