{
  mkShell,
  rust-analyzer,
  rustfmt,
  clippy,
  cargo-expand,
  cargo,
  rustc,
  rustPlatform,
}:
mkShell {
  nativeBuildInputs = [
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy
    cargo-expand
  ];

  env.RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}
