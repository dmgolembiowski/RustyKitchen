use inline_python::{python, pyo3};

pub fn test() {
    let _context = inline_python::Context::new();
    python! {
        #![context = &_context]
        import asyncio
        def testing():
            async def coroutine():
                await asyncio.sleep(4)
                print("Sleeping completed")
            asyncio.run(coroutine())
            return 12
        output = testing()
    }
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let foo: Option<i32> = _context.get_global(py, "output").unwrap();
    assert_eq!(foo, Some(12));
}
