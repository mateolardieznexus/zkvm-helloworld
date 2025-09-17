use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};
use tracing::{info, info_span};

const PACKAGE: &str = "guest";

fn main() {
    // Initialize pretty logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_ansi(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .init();

    info!("🚀 Starting Nexus zkVM Hello World");
    
    let _span = info_span!("compilation").entered();
    info!("📦 Compiling guest program...");
    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");
    info!("✅ Guest program compiled successfully!");

    let elf = prover.elf.clone(); // save elf for use with test verification

    let _span = info_span!("proving").entered();
    info!("🔍 Proving execution of VM...");
    let (view, proof) = prover
        .prove_with_input::<u32, u32>(&3, &5)
        .expect("failed to prove program"); // x = 5, y = 3

    assert_eq!(view.exit_code().expect("failed to retrieve exit code"), 0);

    let output: u32 = view
        .public_output::<u32>()
        .expect("failed to retrieve public output");
    assert_eq!(output, 15); // z = 15

    info!("🎯 Program output: {}", output);
    
    // Pretty print the guest logs
    let logs = view.logs().expect("failed to retrieve debug logs");
    if !logs.is_empty() {
        info!("📋 Guest Program Logs:");
        for log in logs {
            info!("  └─ {}", log.trim());
        }
    }

    let _span = info_span!("verification").entered();
    info!("🔐 Verifying execution...");
    proof
        .verify_expected::<u32, u32>(
            &5,   // x = 5
            0,    // exit code = 0
            &15,  // z = 15
            &elf, // expected elf (program binary)
            &[],  // no associated data,
        )
        .expect("failed to verify proof");

    info!("🎉 Verification succeeded!");
    info!("✨ zkVM Hello World completed successfully!");
}