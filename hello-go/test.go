package main

import (
	"os"
	"time"

	"github.com/second-state/WasmEdge-go/wasmedge"
)

func hello(_ any, _ *wasmedge.CallingFrame, _ []any) ([]any, wasmedge.Result) {
	return nil, wasmedge.Result_Success
}

func main() {
	conf := wasmedge.NewConfigure(wasmedge.WASI)
	defer conf.Release()
	vm := wasmedge.NewVMWithConfig(conf)
	defer vm.Release()

	wasi := vm.GetImportModule(wasmedge.WASI)
	wasi.InitWasi(
		nil,
		os.Environ(),
		[]string{".:."},
	)

	module := wasmedge.NewModule("env")
	defer module.Release()
	f := wasmedge.NewFunction(wasmedge.NewFunctionType(
		[]wasmedge.ValType{},
		[]wasmedge.ValType{}), hello, nil, 0)
	module.AddFunction("hello", f)
	vm.RegisterModule(module)

	vm.LoadWasmFile("/Volumes/Dev/secondstate/me/mem-test/target/wasm32-wasi/release/hello.wasm")
	vm.Validate()
	vm.Instantiate()

	start := time.Now()
	timeElapsed := time.Since(start).Seconds()
	for timeElapsed < 3600 + 1800 {
		// time.Sleep(100 * time.Millisecond)
		vm.Execute("hello")
		timeElapsed = time.Since(start).Seconds()
	}
	
	time.Sleep(300 * time.Second)
}
