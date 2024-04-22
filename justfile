sync-to-blitz:
  rsync -avPzu --delete-during --progress src/ admin@192.168.8.242:/home/admin/dev

# format all Nix files
format:
  alejandra .
 
