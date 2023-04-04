use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    host_function, params, Caller, ImportObjectBuilder, Module, VmBuilder, WasmValue,
};

#[host_function]
fn hello(_: Caller, _: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Hello, world!");

    Ok(vec![])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load wasm module from file
    let module = Module::from_file(
        None,
        "/Users/sam/workspace/rust/mem-test/hello/target/wasm32-wasi/release/hello.wasm",
    )?;

    // create import object
    let import = ImportObjectBuilder::new()
        .with_func::<(), ()>("hello", hello)?
        .build("env")?;

    // create vm
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::new().wasi(true))
        .build()?;
    let mut vm = VmBuilder::new().with_config(config).build()?;

    // init default wasi module
    let wasi_module = vm
        .wasi_module_mut()
        .ok_or("Failed to get the default wasi module")?;
    let vec_envs: Vec<String> = std::env::vars()
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    let envs = vec_envs.iter().map(|s| s.as_str()).collect();
    let preopens = vec![".:."];
    wasi_module.initialize(None, Some(envs), Some(preopens));

    // register import object and wasm module
    let vm = vm
        .register_import_module(import)?
        .register_module(None, module)?;

    let mut round = 1;
    loop {
        println!("Round_{}", round);
        vm.run_func(None, "hello", params![])?;
        round += 1;
    }

    Ok(())
}
