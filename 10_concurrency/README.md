# 10 - Concurrency ⭐⭐⭐⭐

## 🧵 Threads, Channels, Arc, Mutex

## 🌍 Real-World Use Cases

### **Web Servers (Actix, Axum, Hyper)**
```rust
// Handle 10,000 concurrent connections
async fn handle_request(req: Request) -> Response {
    // Each request runs on different thread
    // No data races thanks to Rust's type system
}
```
**ใช้โดย:** Discord, Cloudflare, AWS Lambda

### **Game Servers**
```rust
// Game state shared between players
let game_state = Arc::new(Mutex::new(GameState::new()));

for player in players {
    let state = Arc::clone(&game_state);
    thread::spawn(move || {
        // Each player thread can safely update game state
        let mut state = state.lock().unwrap();
        state.update_player(player.id, player.action);
    });
}
```
**ใช้โดย:** Riot Games, Embark Studios

### **Data Processing Pipelines**
```rust
// Process 1TB of data in parallel
let data = load_big_dataset();
let results: Vec<_> = data
    .par_chunks(1000)  // Rayon crate
    .map(|chunk| heavy_computation(chunk))
    .collect();
```
**ใช้โดย:** Dropbox (file sync), npm (registry)

### **Real-time Systems**
```rust
// Message queue with channels
let (tx, rx) = mpsc::channel();

// Producer thread
thread::spawn(move || {
    for msg in messages {
        tx.send(msg).unwrap();
    }
});

// Consumer thread
thread::spawn(move || {
    for msg in rx {
        process_message(msg);
    }
});
```
**ใช้โดย:** Kafka clients, RabbitMQ, Redis

### **IoT & Embedded**
```rust
// Sensor data collection
let sensor_data = Arc::new(Mutex::new(Vec::new()));

// Thread 1: Read sensor
let data1 = Arc::clone(&sensor_data);
thread::spawn(move || {
    loop {
        let reading = read_temperature_sensor();
        data1.lock().unwrap().push(reading);
    }
});

// Thread 2: Process data
let data2 = Arc::clone(&sensor_data);
thread::spawn(move || {
    loop {
        thread::sleep(Duration::from_secs(60));
        let avg = calculate_average(&data2.lock().unwrap());
        send_to_cloud(avg);
    }
});
```

## 🚀 Performance Comparison

| Language | Concurrent Connections | Memory Safety |
|----------|----------------------|---------------|
| Rust | 1M+ | Compile-time guarantee |
| Go | 100K+ | Runtime GC pauses |
| C++ | 1M+ | Manual (error-prone) |
| Node.js | 10K | Single-threaded |

## 📝 แบบฝึกหัด

ดูโค้ดใน `src/lib.rs`