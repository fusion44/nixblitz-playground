rust_src := "./packages"

set positional-arguments

sync-to-blitz:
  # PI
  rsync -avPz --delete-during --progress src/ admin@192.168.8.242:/home/admin/dev/sys
  rsync -avPz --delete-during --progress ../api/nixosify/ admin@192.168.8.242:/home/admin/dev/api
  rsync -avPz --delete-during --progress ../web/nixosify/ admin@192.168.8.242:/home/admin/dev/web
  rsync -avPz --delete-during --progress --exclude="history.txt" src/configs/nushell/ admin@192.168.8.242:/home/admin/.config/nushell

  # vm
  rsync -avPze 'ssh -p 10022 -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no' --delete-during --progress src/ admin@localhost:/home/admin/dev/sys
  rsync -avPze 'ssh -p 10022 -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no' --delete-during --progress ../api/nixosify/ admin@localhost:/home/admin/dev/api
  rsync -avPze 'ssh -p 10022 -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no' --delete-during --progress ../web/nixosify/ admin@localhost:/home/admin/dev/web
  rsync -avPze 'ssh -p 10022 -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no' --delete-during --progress --exclude="history.txt" src/configs/nushell/ admin@localhost:/home/admin/.config/nushell

# format all Nix files
format:
  alejandra	.

lint: 
  cd {{rust_src}} && cargo check

# run the CLI, any args are passed to the CLI unaltered 
run-cli *args='':
  cd {{rust_src}} && cargo run $@

# builds the current config as a qemu vm
vm-build:
	cd src && nixos-rebuild build-vm --flake .#tbnixvm

# runs the current qemu vm
vm-run:
	export QEMU_NET_OPTS="hostfwd=tcp::18444-:18444,hostfwd=tcp::10022-:22,hostfwd=tcp::8080-:80,hostfwd=tcp::9735-:9735" && ./src/result/bin/run-tbnixvm-vm

# ssh into the currently running qemu vm
vm-ssh:
	ssh -oUserKnownHostsFile=/dev/null -oStrictHostKeyChecking=no admin@localhost -p 10022 

# resets the vm by deleting the tbnix_vm.qcow2 file
vm-reset:
	rm -i tbnixvm.qcow2


