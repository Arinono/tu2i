{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };

        tu2i = naersk-lib.buildPackage {
          src = ./.;

          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          OPENSSL_NO_VENDOR = "1";
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        };

        certs = pkgs.cacert;

        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "tu2i";
          tag = "latest";
          contents = [
            tu2i
            certs
          ];
          config = {
            Cmd = [ "${tu2i}/bin/tu2i" ];
            Env = [
              "SSL_CERT_FILE=${certs}/etc/ssl/certs/ca-bundle.crt"
            ];
          };
        };
      in
      with pkgs;
      {
        packages = {
          inherit tu2i dockerImage;
          default = tu2i;
        };
        devShell = mkShell {
          buildInputs = [ tu2i cargo rustc rustfmt rustPackages.clippy dive just ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}

