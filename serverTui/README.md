src/
  main.rs              // init terminal, run app
  app.rs               // App struct (state) + update methods
  ui.rs                // draw(App)
  event.rs             // key handling + tick events
  services/
    mod.rs
    docker.rs          // fetch docker ps
    pm2.rs             // fetch pm2 list
  types.rs             // shared structs (ProcessRow, DockerRow)
