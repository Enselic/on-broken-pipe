// Copyright (c) 2026 Martin Nordholts
// SPDX-License-Identifier: MIT

#![feature(on_broken_pipe)]

#[std::io::on_broken_pipe]
fn on_broken_pipe() -> std::io::OnBrokenPipe {
    std::io::OnBrokenPipe::Inherit
}
