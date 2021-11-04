docker run\
  --env BIND_PORT="8080"\
  --env BIND_HOST="0.0.0.0"\
  --env RUNTIME_DATABASE_URL="/var/local/database.db"\
  -p 8080:8080\
  abc:latest