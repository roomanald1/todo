job "todo" {
  datacenters = ["dc1"]
  type        = "service"

  group "app" {
    count = 1

    network {
      port "http" {
        to = 8000
      }
    }

    service {
      name = "todo"
      port = "http"

      check {
        type     = "http"
        path     = "/health"
        interval = "10s"
        timeout  = "2s"
      }
    }

    task "server" {
      driver = "docker"

      config {
        image = "ronnieday/todo:0.0.4"
        ports = ["http"]
        load  = "ronnieday/todo:0.0.4"  # Load the local image
      }

      resources {
        cpu    = 500  # 500 MHz
        memory = 256  # 256MB
      }
    }
  }
}