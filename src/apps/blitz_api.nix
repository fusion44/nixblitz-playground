{
  services.blitz-api = {
    enable = true;
    openFirewall = true;
    host = "0.0.0.0";
    ln.connectionType = "lnd_grpc";
    # logLevel = "TRACE";
    dotEnvFile = "/var/lib/blitz_api/.env";
    passwordFile = "/run/keys/login_password";
    # rootPath = "/api";
  };
}
