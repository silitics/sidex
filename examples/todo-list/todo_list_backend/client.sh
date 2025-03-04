#!/usr/bin/env bash

# curl -X POST -H "Content-Type: application/json" \
#     -d '{ "name": "Test Task" }' \
#     http://127.0.0.1:3030/api/v1/rpc/TodoList.create

curl -X POST -H "Content-Type: application/json" \
    -d '{ "task": "1c96524d-8183-48ed-97ba-94f35a71cebe" }' \
    http://127.0.0.1:3030/api/v1/rpc/TodoList.delete