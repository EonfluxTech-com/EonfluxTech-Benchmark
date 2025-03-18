// Credits to https://github.com/EonfluxTech/EonfluxTech-Benchmark-Tool contributed by @Dyplay
// 2025-03-18 12:00 AM copyright EonfluxTech.com 2025

use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType, EnableLineWrap},
};
use sysinfo::{CpuExt, System, SystemExt, DiskExt};
use std::io::{stdout, Write, Read, Seek, SeekFrom};
use std::time::{Duration, Instant};
use std::fs::OpenOptions;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use rand::Rng;
use chrono::Local;

const MEMORY_TEST_SIZE: usize = 1024 * 1024 * 256; // 256 MB
const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB
const DISK_TEST_SIZE: usize = 1024 * 1024 * 512; // 512 MB
const DISK_CHUNK_SIZE: usize = 1024 * 1024; // 1 MB chunks for disk operations

#[derive(Default)]
struct BenchmarkResults {
    cpu_score: f64,
    memory_read_speed: f64,
    memory_write_speed: f64,
    memory_latency: f64,
    disk_read_speed: f64,
    disk_write_speed: f64,
    disk_iops: f64,
}

struct BenchmarkTool {
    sys: System,
}

impl BenchmarkTool {
    fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    fn display_back_button(&self) {
        println!("\n{}", "═══════════════════════════════════════".bright_blue());
        println!("{} {}", "◄".bright_red(), "Press 'B' or ESC to go back to main menu");
    }

    fn wait_for_back(&self) -> bool {
        if let Ok(Event::Key(key_event)) = event::read() {
            matches!(key_event.code, KeyCode::Char('b') | KeyCode::Char('B') | KeyCode::Esc)
        } else {
            false
        }
    }

    fn clear_screen(&self) {
        // Clear the entire screen and scroll buffer
        print!("\x1B[2J\x1B[3J\x1B[1;1H");
        stdout().flush().unwrap();
        
        execute!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            terminal::DisableLineWrap,
        ).unwrap();
    }

    fn display_menu(&self) -> std::io::Result<()> {
        self.clear_screen();

        println!("{}", "╔══════════════════════════════════════╗".bright_blue());
        println!("{}", "║     EonfluxTech Benchmark Tool       ║".bright_blue());
        println!("{}", "╚══════════════════════════════════════╝".bright_blue());
        println!();
        println!("{}:", "Available Options".bright_yellow());
        println!("1. {} {}", "►".bright_green(), "System Information");
        println!("2. {} {}", "►".bright_green(), "CPU Benchmark");
        println!("3. {} {}", "►".bright_green(), "Memory Benchmark");
        println!("4. {} {}", "►".bright_green(), "Disk Benchmark");
        println!("5. {} {}", "►".bright_green(), "Full System Benchmark");
        println!("6. {} {}", "►".bright_red(), "Exit");
        println!();
        println!("Press the number of your choice...");
        Ok(())
    }

    fn show_system_info(&mut self) {
        loop {
            self.sys.refresh_all();
            self.clear_screen();
            
            println!("{}", "╔══════════════════════════════════════╗".bright_blue());
            println!("{}", "║        System Information            ║".bright_blue());
            println!("{}", "╚══════════════════════════════════════╝".bright_blue());
            println!();
            
            // OS Info
            println!("{}", "Operating System:".bright_yellow());
            println!("► Name: {}", self.sys.name().unwrap_or_else(|| "Unknown".to_string()));
            println!("► Version: {}", self.sys.os_version().unwrap_or_else(|| "Unknown".to_string()));
            println!("► Kernel Version: {}", self.sys.kernel_version().unwrap_or_else(|| "Unknown".to_string()));
            println!();

            // CPU Info
            println!("{}", "CPU Information:".bright_yellow());
            if let Some(cpu) = self.sys.cpus().first() {
                println!("► Brand: {}", cpu.brand());
                println!("► Cores: {}", self.sys.cpus().len());
                println!("► Frequency: {} MHz", cpu.frequency());
                println!("► CPU Usage: {:.1}%", cpu.cpu_usage());
            }
            println!();

            // Memory Info
            println!("{}", "Memory Information:".bright_yellow());
            println!("► Total RAM: {:.2} GB", self.sys.total_memory() as f64 / 1024.0 / 1024.0);
            println!("► Available RAM: {:.2} GB", self.sys.available_memory() as f64 / 1024.0 / 1024.0);
            println!("► Used RAM: {:.2} GB", (self.sys.total_memory() - self.sys.available_memory()) as f64 / 1024.0 / 1024.0);
            println!("► Memory Usage: {:.1}%", (1.0 - (self.sys.available_memory() as f64 / self.sys.total_memory() as f64)) * 100.0);

            self.display_back_button();
            println!("\nPress 'R' to refresh information");

            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char('b') | KeyCode::Char('B') | KeyCode::Esc => break,
                    KeyCode::Char('r') | KeyCode::Char('R') => continue,
                    _ => {}
                }
            }
        }
    }

    fn run_cpu_benchmark(&mut self) {
        loop {
            self.clear_screen();

            println!("{}", "╔══════════════════════════════════════╗".bright_blue());
            println!("{}", "║          CPU Benchmark               ║".bright_blue());
            println!("{}", "╚══════════════════════════════════════╝".bright_blue());
            println!();

            // Initialize progress bar
            let pb = ProgressBar::new(100);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"));

            // Prepare benchmark parameters
            let duration = Duration::from_secs(5);
            let start_time = Instant::now();
            let mut operations = 0u64;

            // CPU intensive calculations
            println!("{}", "Running CPU stress test...".bright_yellow());
            while start_time.elapsed() < duration {
                // Complex mathematical operations
                let mut rng = rand::thread_rng();
                let n: f64 = rng.gen();
                let _result = (0..1000).fold(n, |acc, _| {
                    (acc.sqrt().sin() * acc.cos()).exp()
                });
                
                operations += 1;
                if operations % 100 == 0 {
                    pb.set_position((start_time.elapsed().as_secs_f64() / duration.as_secs_f64() * 100.0) as u64);
                }
            }
            pb.finish_with_message("Benchmark complete");

            // Calculate score
            let score = operations as f64 / duration.as_secs_f64();
            println!();
            println!("Benchmark Results:");
            println!("► Operations per second: {:.2}", score);
            println!("► Total operations: {}", operations);
            println!("► Time elapsed: {:.2} seconds", start_time.elapsed().as_secs_f64());
            
            // CPU Usage after benchmark
            self.sys.refresh_cpu();
            if let Some(cpu) = self.sys.cpus().first() {
                println!("► Current CPU Usage: {:.1}%", cpu.cpu_usage());
            }

            // Performance Rating
            let rating = match score {
                s if s > 1_000_000.0 => "Excellent".bright_green(),
                s if s > 750_000.0 => "Very Good".bright_blue(),
                s if s > 500_000.0 => "Good".bright_yellow(),
                s if s > 250_000.0 => "Fair".bright_yellow(),
                _ => "Needs Improvement".bright_red(),
            };
            println!("► Performance Rating: {}", rating);
            println!();

            self.display_back_button();
            println!("\nPress 'R' to run the benchmark again");

            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char('b') | KeyCode::Char('B') | KeyCode::Esc => break,
                    KeyCode::Char('r') | KeyCode::Char('R') => continue,
                    _ => {}
                }
            }
        }
    }

    fn run_memory_benchmark(&mut self) {
        loop {
            self.clear_screen();

            println!("{}", "╔══════════════════════════════════════╗".bright_blue());
            println!("{}", "║        Memory Benchmark              ║".bright_blue());
            println!("{}", "╚══════════════════════════════════════╝".bright_blue());
            println!();

            // Initialize progress bars
            let pb = ProgressBar::new(100);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"));

            println!("{}", "Allocating memory for test...".bright_yellow());
            let mut data = vec![0u8; MEMORY_TEST_SIZE];
            
            // Write speed test
            println!("{}", "\nTesting write speed...".bright_yellow());
            let write_start = Instant::now();
            let mut rng = rand::thread_rng();
            
            for (i, chunk) in data.chunks_mut(CHUNK_SIZE).enumerate() {
                // Fill with random data
                for byte in chunk.iter_mut() {
                    *byte = rng.gen();
                }
                pb.set_position((i * 100 / (MEMORY_TEST_SIZE / CHUNK_SIZE)) as u64);
            }
            let write_duration = write_start.elapsed();
            let write_speed = MEMORY_TEST_SIZE as f64 / write_duration.as_secs_f64() / 1024.0 / 1024.0;
            pb.finish_with_message("Write test complete");

            // Read speed test
            println!("{}", "\nTesting read speed...".bright_yellow());
            pb.set_position(0);
            let read_start = Instant::now();
            let mut checksum = 0u64;

            for (i, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
                // Read and perform a simple checksum
                for &byte in chunk {
                    checksum = checksum.wrapping_add(byte as u64);
                }
                pb.set_position((i * 100 / (MEMORY_TEST_SIZE / CHUNK_SIZE)) as u64);
            }
            let read_duration = read_start.elapsed();
            let read_speed = MEMORY_TEST_SIZE as f64 / read_duration.as_secs_f64() / 1024.0 / 1024.0;
            pb.finish_with_message("Read test complete");

            // Display results
            println!("\nMemory Benchmark Results:");
            println!("► Write Speed: {:.2} MB/s", write_speed);
            println!("► Read Speed: {:.2} MB/s", read_speed);
            println!("► Average Speed: {:.2} MB/s", (write_speed + read_speed) / 2.0);
            
            // Memory latency test
            println!("\nTesting memory latency...");
            let latency_start = Instant::now();
            let mut pointer = 0usize;
            for _ in 0..1_000_000 {
                pointer = data[pointer] as usize % MEMORY_TEST_SIZE;
            }
            let latency = latency_start.elapsed().as_nanos() as f64 / 1_000_000.0;
            println!("► Memory Latency: {:.2} ns", latency);

            // Performance Rating
            let avg_speed = (write_speed + read_speed) / 2.0;
            let rating = match avg_speed {
                s if s > 10000.0 => "Excellent".bright_green(),
                s if s > 7500.0 => "Very Good".bright_blue(),
                s if s > 5000.0 => "Good".bright_yellow(),
                s if s > 2500.0 => "Fair".bright_yellow(),
                _ => "Needs Improvement".bright_red(),
            };
            println!("► Performance Rating: {}", rating);
            println!();

            // Memory Usage
            self.sys.refresh_memory();
            println!("Current Memory Status:");
            println!("► Used Memory: {:.2} GB", (self.sys.total_memory() - self.sys.available_memory()) as f64 / 1024.0 / 1024.0);
            println!("► Available Memory: {:.2} GB", self.sys.available_memory() as f64 / 1024.0 / 1024.0);
            println!("► Memory Usage: {:.1}%", (1.0 - (self.sys.available_memory() as f64 / self.sys.total_memory() as f64)) * 100.0);

            self.display_back_button();
            println!("\nPress 'R' to run the benchmark again");

            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char('b') | KeyCode::Char('B') | KeyCode::Esc => break,
                    KeyCode::Char('r') | KeyCode::Char('R') => continue,
                    _ => {}
                }
            }
        }
    }

    fn run_disk_benchmark(&mut self) {
        loop {
            self.clear_screen();

            println!("{}", "╔══════════════════════════════════════╗".bright_blue());
            println!("{}", "║         Disk Benchmark               ║".bright_blue());
            println!("{}", "╚══════════════════════════════════════╝".bright_blue());
            println!();

            let test_file = "disk_benchmark_test.tmp";
            let pb = ProgressBar::new(100);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"));

            // Sequential Write Test
            println!("{}", "Running Sequential Write Test...".bright_yellow());
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(test_file)
                .unwrap();

            let write_start = Instant::now();
            let mut rng = rand::thread_rng();
            let mut buffer = vec![0u8; DISK_CHUNK_SIZE];
            let mut total_written = 0;

            while total_written < DISK_TEST_SIZE {
                rng.fill(&mut buffer[..]);
                file.write_all(&buffer).unwrap();
                total_written += DISK_CHUNK_SIZE;
                pb.set_position((total_written as f64 / DISK_TEST_SIZE as f64 * 100.0) as u64);
            }
            file.sync_all().unwrap();
            let seq_write_speed = DISK_TEST_SIZE as f64 / write_start.elapsed().as_secs_f64() / 1024.0 / 1024.0;
            pb.finish_with_message("Sequential write complete");

            // Sequential Read Test
            println!("\n{}", "Running Sequential Read Test...".bright_yellow());
            pb.set_position(0);
            file.seek(SeekFrom::Start(0)).unwrap();
            let read_start = Instant::now();
            let mut total_read = 0;
            let mut buffer = vec![0u8; DISK_CHUNK_SIZE];

            while total_read < DISK_TEST_SIZE {
                file.read_exact(&mut buffer).unwrap();
                total_read += DISK_CHUNK_SIZE;
                pb.set_position((total_read as f64 / DISK_TEST_SIZE as f64 * 100.0) as u64);
            }
            let seq_read_speed = DISK_TEST_SIZE as f64 / read_start.elapsed().as_secs_f64() / 1024.0 / 1024.0;
            pb.finish_with_message("Sequential read complete");

            // Random Read/Write Test
            println!("\n{}", "Running Random Access Test...".bright_yellow());
            pb.set_position(0);
            let random_start = Instant::now();
            let mut operations = 0;
            let num_random_ops = 1000;

            for i in 0..num_random_ops {
                let pos = rng.gen_range(0..DISK_TEST_SIZE - DISK_CHUNK_SIZE) as u64;
                file.seek(SeekFrom::Start(pos)).unwrap();
                
                if i % 2 == 0 {
                    file.read_exact(&mut buffer).unwrap();
                } else {
                    rng.fill(&mut buffer[..]);
                    file.write_all(&buffer).unwrap();
                }
                operations += 1;
                pb.set_position((operations as f64 / num_random_ops as f64 * 100.0) as u64);
            }
            let random_iops = operations as f64 / random_start.elapsed().as_secs_f64();
            pb.finish_with_message("Random access test complete");

            // Clean up
            drop(file);
            std::fs::remove_file(test_file).unwrap();

            // Display Results
            println!("\nDisk Benchmark Results:");
            println!("► Sequential Write Speed: {:.2} MB/s", seq_write_speed);
            println!("► Sequential Read Speed: {:.2} MB/s", seq_read_speed);
            println!("► Random Access Speed: {:.2} IOPS", random_iops);
            println!("► Average Sequential Speed: {:.2} MB/s", (seq_write_speed + seq_read_speed) / 2.0);

            // Performance Rating
            let avg_speed = (seq_write_speed + seq_read_speed) / 2.0;
            let rating = match avg_speed {
                s if s > 1000.0 => "Excellent".bright_green(),
                s if s > 500.0 => "Very Good".bright_blue(),
                s if s > 250.0 => "Good".bright_yellow(),
                s if s > 100.0 => "Fair".bright_yellow(),
                _ => "Needs Improvement".bright_red(),
            };
            println!("► Performance Rating: {}", rating);

            // Disk Information
            self.sys.refresh_disks();
            println!("\nDisk Information:");
            for disk in self.sys.disks() {
                println!("► Disk Name: {}", disk.name().to_string_lossy());
                println!("  - Total Space: {:.2} GB", disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0);
                println!("  - Available Space: {:.2} GB", disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0);
                println!("  - File System: {}", String::from_utf8_lossy(disk.file_system()));
            }

            self.display_back_button();
            println!("\nPress 'R' to run the benchmark again");

            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char('b') | KeyCode::Char('B') | KeyCode::Esc => break,
                    KeyCode::Char('r') | KeyCode::Char('R') => continue,
                    _ => {}
                }
            }
        }
    }

    fn run_full_benchmark(&mut self) -> std::io::Result<()> {
        loop {
            self.clear_screen();

            println!("{}", "╔══════════════════════════════════════╗".bright_blue());
            println!("{}", "║      Full System Benchmark           ║".bright_blue());
            println!("{}", "╚══════════════════════════════════════╝".bright_blue());
            println!();
            
            let mut results = BenchmarkResults::default();
            let start_time = Local::now();

            // System Information
            println!("{}", "System Information:".bright_yellow());
            println!("► OS: {} {}", 
                self.sys.name().unwrap_or_else(|| "Unknown".to_string()),
                self.sys.os_version().unwrap_or_else(|| "Unknown".to_string()));
            if let Some(cpu) = self.sys.cpus().first() {
                println!("► CPU: {} ({} cores)", cpu.brand(), self.sys.cpus().len());
            }
            println!("► RAM: {:.2} GB", self.sys.total_memory() as f64 / 1024.0 / 1024.0);
            println!();

            // Progress bars setup
            let m = MultiProgress::new();
            let sty = ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({msg})")
                .unwrap()
                .progress_chars("#>-");

            // CPU Benchmark
            println!("{}", "Running CPU Benchmark...".bright_yellow());
            let cpu_pb = m.add(ProgressBar::new(100));
            cpu_pb.set_style(sty.clone());

            let duration = Duration::from_secs(5);
            let cpu_start = Instant::now();
            let mut operations = 0u64;

            while cpu_start.elapsed() < duration {
                let mut rng = rand::thread_rng();
                let n: f64 = rng.gen();
                let _result = (0..1000).fold(n, |acc, _| {
                    (acc.sqrt().sin() * acc.cos()).exp()
                });
                
                operations += 1;
                if operations % 100 == 0 {
                    cpu_pb.set_position((cpu_start.elapsed().as_secs_f64() / duration.as_secs_f64() * 100.0) as u64);
                }
            }
            results.cpu_score = operations as f64 / duration.as_secs_f64();
            cpu_pb.finish_with_message("Complete");

            // Memory Benchmark
            println!("\n{}", "Running Memory Benchmark...".bright_yellow());
            let mem_pb = m.add(ProgressBar::new(100));
            mem_pb.set_style(sty.clone());

            let mut data = vec![0u8; MEMORY_TEST_SIZE];
            
            // Write test
            let write_start = Instant::now();
            let mut rng = rand::thread_rng();
            for (i, chunk) in data.chunks_mut(CHUNK_SIZE).enumerate() {
                for byte in chunk.iter_mut() {
                    *byte = rng.gen();
                }
                mem_pb.set_position((i * 100 / (MEMORY_TEST_SIZE / CHUNK_SIZE)) as u64);
            }
            results.memory_write_speed = MEMORY_TEST_SIZE as f64 / write_start.elapsed().as_secs_f64() / 1024.0 / 1024.0;

            // Read test
            mem_pb.reset();
            let read_start = Instant::now();
            let mut checksum = 0u64;
            for (i, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
                for &byte in chunk {
                    checksum = checksum.wrapping_add(byte as u64);
                }
                mem_pb.set_position((i * 100 / (MEMORY_TEST_SIZE / CHUNK_SIZE)) as u64);
            }
            results.memory_read_speed = MEMORY_TEST_SIZE as f64 / read_start.elapsed().as_secs_f64() / 1024.0 / 1024.0;
            
            // Latency test
            let latency_start = Instant::now();
            let mut pointer = 0usize;
            for _ in 0..1_000_000 {
                pointer = data[pointer] as usize % MEMORY_TEST_SIZE;
            }
            results.memory_latency = latency_start.elapsed().as_nanos() as f64 / 1_000_000.0;
            mem_pb.finish_with_message("Complete");

            // Disk Benchmark
            println!("\n{}", "Running Disk Benchmark...".bright_yellow());
            let disk_pb = m.add(ProgressBar::new(100));
            disk_pb.set_style(sty.clone());

            let test_file = "disk_benchmark_test.tmp";
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(test_file)?;

            // Sequential Write
            let mut buffer = vec![0u8; DISK_CHUNK_SIZE];
            let mut total_written = 0;
            let write_start = Instant::now();

            while total_written < DISK_TEST_SIZE {
                rng.fill(&mut buffer[..]);
                file.write_all(&buffer)?;
                total_written += DISK_CHUNK_SIZE;
                disk_pb.set_position((total_written as f64 / DISK_TEST_SIZE as f64 * 100.0) as u64);
            }
            file.sync_all()?;
            results.disk_write_speed = DISK_TEST_SIZE as f64 / write_start.elapsed().as_secs_f64() / 1024.0 / 1024.0;

            // Sequential Read
            disk_pb.reset();
            file.seek(SeekFrom::Start(0))?;
            let read_start = Instant::now();
            let mut total_read = 0;

            while total_read < DISK_TEST_SIZE {
                file.read_exact(&mut buffer)?;
                total_read += DISK_CHUNK_SIZE;
                disk_pb.set_position((total_read as f64 / DISK_TEST_SIZE as f64 * 100.0) as u64);
            }
            results.disk_read_speed = DISK_TEST_SIZE as f64 / read_start.elapsed().as_secs_f64() / 1024.0 / 1024.0;

            // Random IO
            disk_pb.reset();
            let random_start = Instant::now();
            let mut operations = 0;
            let num_random_ops = 1000;

            for i in 0..num_random_ops {
                let pos = rng.gen_range(0..DISK_TEST_SIZE - DISK_CHUNK_SIZE) as u64;
                file.seek(SeekFrom::Start(pos))?;
                
                if i % 2 == 0 {
                    file.read_exact(&mut buffer)?;
                } else {
                    rng.fill(&mut buffer[..]);
                    file.write_all(&buffer)?;
                }
                operations += 1;
                disk_pb.set_position((operations as f64 / num_random_ops as f64 * 100.0) as u64);
            }
            results.disk_iops = operations as f64 / random_start.elapsed().as_secs_f64();
            disk_pb.finish_with_message("Complete");

            // Clean up
            drop(file);
            std::fs::remove_file(test_file)?;

            // Generate Report
            self.generate_benchmark_report(&results, start_time)?;
            
            self.display_back_button();
            println!("\nPress 'R' to run the full benchmark again");

            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char('b') | KeyCode::Char('B') | KeyCode::Esc => break,
                    KeyCode::Char('r') | KeyCode::Char('R') => continue,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn generate_benchmark_report(&self, results: &BenchmarkResults, start_time: chrono::DateTime<Local>) -> std::io::Result<()> {
        self.clear_screen();

        println!("{}", "╔══════════════════════════════════════╗".bright_blue());
        println!("{}", "║     System Benchmark Report          ║".bright_blue());
        println!("{}", "╚══════════════════════════════════════╝".bright_blue());
        println!();

        println!("Test completed at: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        println!("Duration: {:.2} minutes", Local::now().signed_duration_since(start_time).num_minutes());
        println!();

        // CPU Results
        println!("{}", "CPU Performance:".bright_yellow());
        println!("► Operations/second: {:.2}", results.cpu_score);
        let cpu_rating = match results.cpu_score {
            s if s > 1_000_000.0 => "Excellent".bright_green(),
            s if s > 750_000.0 => "Very Good".bright_blue(),
            s if s > 500_000.0 => "Good".bright_yellow(),
            s if s > 250_000.0 => "Fair".bright_yellow(),
            _ => "Needs Improvement".bright_red(),
        };
        println!("► Rating: {}", cpu_rating);
        println!();

        // Memory Results
        println!("{}", "Memory Performance:".bright_yellow());
        println!("► Read Speed: {:.2} MB/s", results.memory_read_speed);
        println!("► Write Speed: {:.2} MB/s", results.memory_write_speed);
        println!("► Latency: {:.2} ns", results.memory_latency);
        let mem_rating = match (results.memory_read_speed + results.memory_write_speed) / 2.0 {
            s if s > 10000.0 => "Excellent".bright_green(),
            s if s > 7500.0 => "Very Good".bright_blue(),
            s if s > 5000.0 => "Good".bright_yellow(),
            s if s > 2500.0 => "Fair".bright_yellow(),
            _ => "Needs Improvement".bright_red(),
        };
        println!("► Rating: {}", mem_rating);
        println!();

        // Disk Results
        println!("{}", "Disk Performance:".bright_yellow());
        println!("► Sequential Read: {:.2} MB/s", results.disk_read_speed);
        println!("► Sequential Write: {:.2} MB/s", results.disk_write_speed);
        println!("► Random IO Operations: {:.2} IOPS", results.disk_iops);
        let disk_rating = match (results.disk_read_speed + results.disk_write_speed) / 2.0 {
            s if s > 1000.0 => "Excellent".bright_green(),
            s if s > 500.0 => "Very Good".bright_blue(),
            s if s > 250.0 => "Good".bright_yellow(),
            s if s > 100.0 => "Fair".bright_yellow(),
            _ => "Needs Improvement".bright_red(),
        };
        println!("► Rating: {}", disk_rating);
        println!();

        // Overall System Rating
        println!("{}", "Overall System Rating:".bright_yellow());
        let overall_score = match (
            results.cpu_score > 500_000.0,
            (results.memory_read_speed + results.memory_write_speed) / 2.0 > 5000.0,
            (results.disk_read_speed + results.disk_write_speed) / 2.0 > 250.0
        ) {
            (true, true, true) => "Excellent".bright_green(),
            (true, true, false) | (true, false, true) | (false, true, true) => "Very Good".bright_blue(),
            (true, false, false) | (false, true, false) | (false, false, true) => "Good".bright_yellow(),
            (false, false, false) => "Needs Improvement".bright_red(),
        };
        println!("► Rating: {}", overall_score);

        Ok(())
    }

    fn run(&mut self) -> std::io::Result<()> {
        terminal::enable_raw_mode()?;

        loop {
            self.clear_screen();
            self.display_menu()?;

            if let Event::Key(key_event) = event::read()? {
                self.clear_screen();  // Clear screen before showing new page
                match key_event.code {
                    KeyCode::Char('1') => self.show_system_info(),
                    KeyCode::Char('2') => self.run_cpu_benchmark(),
                    KeyCode::Char('3') => self.run_memory_benchmark(),
                    KeyCode::Char('4') => self.run_disk_benchmark(),
                    KeyCode::Char('5') => self.run_full_benchmark()?,
                    KeyCode::Char('6') | KeyCode::Esc => break,
                    _ => continue,
                }
                self.clear_screen();  // Clear screen after returning from page
            }
        }

        execute!(stdout(), EnableLineWrap)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut tool = BenchmarkTool::new();
    tool.run()?;
    Ok(())
} 