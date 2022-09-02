use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    params, CallingFrame, ImportObjectBuilder, Module, Vm, WasmValue,
};

fn hello(_: &CallingFrame, _: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Hello, world!");

    Ok(vec![])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::new().wasi(true))
        .build()?;

    // create a Vm context
    let mut vm = Vm::new(Some(config))?;

    // get default wasi module
    let mut wasi_module = vm.wasi_module()?;
    let vec_envs: Vec<String> = std::env::vars()
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    let envs = vec_envs.iter().map(|s| s.as_str()).collect();
    let preopens = vec![".:."];
    wasi_module.initialize(None, Some(envs), Some(preopens));

    let import = ImportObjectBuilder::new()
        .with_func::<(), ()>("hello", hello)?
        .build("env")?;
    let vm = vm.register_import_module(import)?;

    let module = Module::from_file(
        None,
        "/Users/sam/workspace/rust/mem-test/hello/target/wasm32-wasi/release/hello.wasm",
    )?;
    let vm = vm.register_module(None, module)?;

    let mut round = 1;
    loop {
        println!("Round_{}", round);
        vm.run_func(None, "hello", params![])?;
        round += 1;
    }

    Ok(())
}
