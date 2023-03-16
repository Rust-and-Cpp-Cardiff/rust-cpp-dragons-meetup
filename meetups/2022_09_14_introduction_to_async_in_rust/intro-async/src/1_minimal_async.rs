#![allow(dead_code, unused_variables)]

use std::future::Future;

fn main() {
    let x = async_fn_syntax();
    let y = async_fn_desugars_to();
}

async fn async_fn_syntax() -> u32 { 
    42
}

fn async_fn_desugars_to() -> impl Future<Output = u32> {
    async { 42 }
}