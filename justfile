import 'justfiles/linting.just'
import 'justfiles/docker.just'

image_name := "ghcr.io/lunchtimecode/free_lunch"


run *args:
    touch local.db
    cargo run -- {{args}}


w:
    cargo watch --ignore 'assets/css' -s 'just run'