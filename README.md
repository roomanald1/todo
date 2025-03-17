## Run in NOMAD ## 

- Build Docker Image
  - docker build -t ronnieday/todo:[version] .
- Run Nomad 
  - nomad agent -dev -bind 0.0.0.0 -log-level=DEBUG -config=/etc/nomad.d/nomad.hcl -network-interface='{{ GetDefaultInterfaces | attr "name" }}'
- Deploy job to nomad
  - cd jobs, nomad job run todo.nomad.hcl

