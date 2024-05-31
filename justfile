rust_src := "./packages"

set positional-arguments

sync-to-blitz:
  rsync -avPzu --delete-during --progress src/ admin@192.168.8.242:/home/admin/dev/sys
  rsync -avPzu --delete-during --progress ../api/nixosify/ admin@192.168.8.242:/home/admin/dev/api
  rsync -avPzu --delete-during --progress ../web/nixosify/ admin@192.168.8.242:/home/admin/dev/web
  rsync -avPzu --delete-during --progress --exclude="history.txt" src/configs/nushell/ admin@192.168.8.242:/home/admin/.config/nushell

# format all Nix files
format:
  alejandra	.
  cd {{rust_src}} && cargo fmt

lint: 
  cd {{rust_src}} && cargo check

# run the CLI, any args are passed to the CLI unaltered 
run-cli *args='':
  cd {{rust_src}} && cargo run $@

run-in-vm:
	cd src && nixos-rebuild build-vm --flake .#devsys && ./result/bin/run-tbnix-vm

