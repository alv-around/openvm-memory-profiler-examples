use eyre::Result;
use openvm_build::{GuestOptions, TargetFilter};
use openvm_sdk::{
    config::{AppConfig, SdkVmConfig},
    Sdk, StdIn,
};
use openvm_stark_sdk::config::FriParameters;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct SomeStruct {
    pub a: u64,
    pub b: u64,
}

pub fn proof_and_verify_fib() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Build the VmConfig with the extensions needed.
    let vm_config = SdkVmConfig::builder()
        .system(Default::default())
        .rv32i(Default::default())
        .rv32m(Default::default())
        .io(Default::default())
        .build();

    let sdk = Sdk;

    // 2a. Build the ELF with guest options and a target filter.
    // INFO: update target_path to the location of your guest code
    let target_path = "openvm-example-fibonacci";
    let guest_opts = GuestOptions::default();
    let target_filter = TargetFilter {
        name: target_path.to_string(),
        kind: "bin".to_string(),
    };
    let elf = sdk.build(guest_opts, target_path, &Some(target_filter))?;
    println!("build complete");

    // ANCHOR: transpilation
    // 3. Transpile the ELF into a VmExe
    let exe = sdk.transpile(elf, vm_config.transpiler())?;
    // ANCHOR_END: transpilation

    // ANCHOR: execution
    // 4. Format your input into StdIn
    let my_input = SomeStruct { a: 1, b: 2 }; // anything that can be serialized
    let mut stdin = StdIn::default();
    stdin.write(&my_input);

    // 5. Run the program
    let output = sdk.execute(exe.clone(), vm_config.clone(), stdin.clone())?;
    println!("public values output: {:?}", output);
    // ANCHOR_END: execution

    // ANCHOR: proof_generation
    // 6. Set app configuration
    let app_log_blowup = 2;
    let app_fri_params = FriParameters::standard_with_100_bits_conjectured_security(app_log_blowup);
    let app_config = AppConfig::new(app_fri_params, vm_config);

    // 7. Commit the exe
    let app_committed_exe = sdk.commit_app_exe(app_fri_params, exe)?;

    // 8. Generate an AppProvingKey
    let app_pk = Arc::new(sdk.app_keygen(app_config)?);

    // 9a. Generate a proof
    let proof = sdk.generate_app_proof(app_pk.clone(), app_committed_exe.clone(), stdin.clone())?;
    println!("proof generated");

    // ANCHOR: verification
    // 10. Verify your program
    let app_vk = app_pk.get_app_vk();
    sdk.verify_app_proof(&app_vk, &proof)?;
    // ANCHOR_END: verification

    Ok(())
}
