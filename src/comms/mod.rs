//
// mod.rs
// Copyright (C) 2021 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

use std::sync::mpsc::*;

pub fn comm_channel<T: Send + 'static>(a: impl Fn(Sender<T>, Receiver<T>) + Send + 'static, b: impl Fn(Sender<T>, Receiver<T>) + Send + 'static) {
    let (atx, arx) = channel();
    let (btx, brx) = channel();
    let ahandle = std::thread::spawn(move || {
        a(atx, brx);
    });
    let bhandle = std::thread::spawn(move || {
        b(btx, arx);
    });
    ahandle.join().unwrap();
    bhandle.join().unwrap();
}
pub fn comm_channel_mitm<T: Send + 'static>(
    a: impl Fn(Sender<T>, Receiver<T>) + Send + 'static,
    b: impl Fn(Sender<T>, Receiver<T>) + Send + 'static,
    m: impl Fn(Sender<T>, Receiver<T>, Sender<T>, Receiver<T>) + Send + 'static,
) {
    let (atx, arx) = channel();
    let (btx, brx) = channel();
    let (matx, marx) = channel();
    let (mbtx, mbrx) = channel();
    let ahandle = std::thread::spawn(move || {
        a(atx, marx);
    });
    let bhandle = std::thread::spawn(move || {
        b(btx, mbrx);
    });
    let mhandle = std::thread::spawn(move || {
        m(matx, arx, mbtx, brx);
    });
    ahandle.join().unwrap();
    bhandle.join().unwrap();
    mhandle.join().unwrap();
}
