# Aide à la création d'un plan de vol
C'est un petit outil fait pour obtenir les coordonnées d'une liste d'addresses
grace à un technique dite de "geocoding". Les coordonnées sont directement
formattées comme demandées dans un plan de vol.

Il a été archivé car l'API de eframe est trop variable.

## L'utiliser
Installez (rust)[rust-lang.org] pour votre système.

Ensuite compilez:
```bash
cargo build --release
```
Puis lancez:
```bash
cargo run --release
```
N'oubliez pas de préciser le chemin vers un fichier d'addresses. Par exemple,
dans ce repo:
```bash
cargo run --release example.txt
```
