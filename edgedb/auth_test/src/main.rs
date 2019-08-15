#![feature(proc_macro_hygiene)]
use inline_python::{python, pyo3};

pub fn login<'s>(
        email: &'s str,
        password: &'s str) -> &'s str {       
    let _context = inline_python::Context::new();
    python! {
        #![context = &_context]
        from __future__ import annotations
        import asyncio
        import os
        import edgedb
        import concurrent.futures
        import functools

        # Use Python-exclusive EdgeDB API
        class Env:
            def __init__(self, host: str, port: int, user: str, database: str, password: str, timeout: float):
                self.host = host
                self.port = port
                self.user = user
                self.database = database
                self.password = password
                self.timeout = timeout
                self.json_string = ''

            @classmethod
            def __new__(cls):
                host = os.getenv("EDGEDB_HOST")
                port = os.getenv("EDGEDB_PORT")
                user = os.getenv("EDGEDB_USER")
                password = os.getenv("EDGEDB_PWD")
                database = os.getenv("EDGEDB_DATABASE")
                timeout = os.getenv("EDGEDB_TIMEOUT")
                return cls.__init__(host, port, user, database, password, timeout)

            def __str__(self):
                return f"edgedb://:{self.password}@{self.host}:{self.port}/{self.database}"            

            @property
            def dsn(self):
                return self.__str__()

        def main() -> str:
            env = Env.__new__()
            email = 'email
            password = str(hash('password))
            lookup = "SELECT User { email, username, privileged, root } FILTER .email = %s AND .password = %s;" % ('email, 'password) 
            future_json = asyncio.future.Future()

            async def run():
                con = await edgedb.async_connect(dsn=env.dsn)
                
            def wrapper():
 
        output = main()
    }
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let foo: Option<i32> = _context.get_global(py, "output").unwrap();
    assert_eq!(foo, Some(12));
    "{username: 'david', email: 'dmgolembiowski@gmail.com'}"
}

fn main() {
    test();
    println!("Hello, world!");
}
