(module
  (func $log (import "imports" "log") (param i32))
  (func (export "addAndLog") (param i32 i32)
    local.get 0
    local.get 1
    i32.add
    call $log))
