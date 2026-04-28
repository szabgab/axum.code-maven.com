docker run --rm -d --name pgserver -e POSTGRES_USER=myuser -e POSTGRES_PASSWORD=secret -e POSTGRES_DB=mydb -p 5432:5432 postgres:18
