//! JavaScript runtime for mir2wasm

// This currently assumes it's running under d8, the V8 shell. We'll
// probably want to make it engine-independent.

let buffer = readbuffer(arguments[0]);

const RUNTIME = {
  panic: function() {
    throw new Error("panic!");
  }
};

let empty_function = function() {}
let module_handler = {
    get: function(target, module_name) {
        if(module_name == "spectest") {
            return {
                print: function(i) {
                    // TODO: support more data types
                    print("(i32.const " + i + ")")
                }
            };
        }
        return new Proxy({}, {
            get: function(target, func_name) {
                if (module_name == "rustrt" && RUNTIME[func_name]) {
                  return RUNTIME[func_name];
                }
                print("Rust requested unknown runtime function "
                      + module_name + "::" + func_name);
                return empty_function;
            }
        });
    }
};
let proxy_ffi = new Proxy({}, module_handler);

WebAssembly.instantiate(buffer, proxy_ffi).then(result => {
  const instance = result.instance;
  instance.exports.rust_entry();
}).catch(err => {
  print(err.stack);
  quit(1);
})
