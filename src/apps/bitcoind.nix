# https://search.nixos.org/flakes?channel=unstable&from=0&size=50&sort=relevance&type=options&query=bitcoin
{
  nix-bitcoin = {
    generateSecrets = true;
    operator = {
      enable = true;
      name = "admin";
    };
  };
  services.bitcoind = {
    enable = true;
    regtest = true;
    txindex = true;
    disablewallet = false;
    rpc.users.blitznix = {
      name = "blitznix";
      # https://jlopp.github.io/bitcoin-core-rpc-auth-generator/
      # test password: test1234
      passwordHMAC = "c3d49b374d453effc172a9d30da0544a$5244587558383e3d1028307176ed89386a26093b10275cd77dca71ebdd3f8b06";
    };
    zmqpubrawblock = "tcp://127.0.0.1:28332";
  };
}
