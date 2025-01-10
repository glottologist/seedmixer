{pkgs ? import <nixpkgs> {}}: let
  lib = pkgs.lib;
in {
  name = "seedmixer";
  ver = "0.1.0";
  homepage = "https://github.com/glottologist/seedmixer";
  description = "A BIP39 seed mixer.";
  license = lib.licences.agpl3Plus;
  maintainers = with lib.maintainers; [
    {
      name = "Jason Ridgway-Taylor";
      email = "jason@glottologist.co.uk";
      github = "glottologist";
    }
  ];
}
