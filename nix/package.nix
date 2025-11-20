{
  dockerTools,
  rustPlatform,
  openssl,
  pkg-config,
  runCommand,
  lib,
}:
let
  rustBin = rustPlatform.buildRustPackage {
    pname = "portfolio";
    version = "0.1.0";
    src = lib.cleanSourceWith {
      src = ../.;
      filter =
        path: type:
        let
          relPath = lib.removePrefix (toString ../. + "/") (toString path);
        in
        lib.any (p: lib.hasPrefix p relPath) [
          "src"
          "Cargo.toml"
          "Cargo.lock"
          "static"
          "templates"
        ];
    };

    cargoLock.lockFile = ../Cargo.lock;

    nativeBuildInputs = [ pkg-config ];
    buildInputs = [ openssl ];
  };
in
dockerTools.buildImage {
  name = rustBin.pname;
  tag = "latest";

  copyToRoot = runCommand "root" { } ''
    mkdir -p $out/bin
    echo "Rust bin output contents:"
    cp ${rustBin}/bin/${rustBin.pname} $out/bin/

    cp -r ${../static} $out/static
    cp -r ${../templates} $out/templates
  '';

  config = {
    Cmd = [ "/bin/${rustBin.pname}" ];
    WorkingDir = "/";
    ExposedPorts = {
      "3000/tcp" = { };
    };
  };
}
