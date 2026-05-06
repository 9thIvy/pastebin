{ pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  buildPhase = ''
    mkdir $out/bin
    cargo build --release --locked
    cp target/relase/${pname} $out/bin
  '';

  installPhase = ''
    mkdir -p $out/lib/systemd/system
    cat > $out/lib/systemd/system/${pname}.service <<'EOF'

[Unit]
Description=Pineapple Pastebin
After=network.target

[Service]
Type=notify
ExecStart=${placeholder "out"}/bin/${pname}

ExecReload=${pkgs.coreutils}/bin/kill -INT $MAINPID \
    && ${placeholder "out"}/bin/${pname}

    ExecStop=${pkgs.coreutils}/bin/kill -9 $MAINPID

Restart=on-failure

[Install]
Wanted-By=multi-user.target

EOF
  '';


}
