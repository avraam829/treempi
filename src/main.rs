use std::process::Command;
use std::time::Instant;

fn main() {
    println!("Запуск последовательной версии:");
    let start_seq = Instant::now();
    let status_seq = Command::new("./target/release/mandelbrot_seq")
        .status()
        .expect("Ошибка запуска последовательной версии");
   
    assert!(status_seq.success());

    println!("\nЗапуск MPI-версии:");
    let start_mpi = Instant::now();
    let status_mpi = Command::new("mpirun")
        .args(["-np", "4", "./target/release/mandelbrot_mpi"])
        .status()
        .expect("Ошибка запуска MPI-версии");
    assert!(status_mpi.success());
}
