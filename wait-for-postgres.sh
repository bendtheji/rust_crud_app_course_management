#!/bin/bash

# Wait until PostgreSQL is ready to accept connections
until psql -h "postgres" -U "postgres_user" -d "course_management_minden_ai" -c '\q'; do
  >&2 echo "PostgreSQL is unavailable - sleeping"
  sleep 1
done

>&2 echo "PostgreSQL is up - executing command"
